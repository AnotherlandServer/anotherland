// Copyright (C) 2025 AnotherlandServer
// 
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
// 
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
// 
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::{fs, path::PathBuf, str::FromStr};
use futures::future::{BoxFuture, FutureExt};

use clap::Parser;
use upk::{types::{ObjectProperty, ScriptObject}, Container, ObjectRef};

#[derive(Parser)]
struct Cli {
    output_dir: String,

    #[arg(long)]
    game_folder: String,

    #[arg(long)]
    package: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let mut container = Container::new(PathBuf::from_str(&cli.game_folder)
        .expect("invalid path")
        .join("UnrealEngine3/AmunGame/CookedPCConsole")
    );

    container.mount_package("Atlas").await
        .expect("failed to mount package");

    container.mount_package("Otherland").await
        .expect("failed to mount package");

    container.mount_package("PathEngine").await
        .expect("failed to mount package");

    container.mount_package("AmunEntry").await
        .expect("failed to mount package");

    container.mount_package("UI_GFx").await
        .expect("failed to mount package");

    container.mount_package("Startup").await
        .expect("failed to mount package");

    container.mount_package(&cli.package).await
        .expect("failed to mount package");

    let outdir = PathBuf::from_str(&cli.output_dir).unwrap();
    fs::create_dir_all(&outdir).expect("failed to create output dir");
    
    if let Some(level) = container.lookup_object("Level:TheWorld/PersistentLevel") {
        for seq in level.children().iter().filter(|p| p.class().name() == "Sequence") {
            println!("Analyzing: {}", seq.name());

            let plot = plot_sequence(&container, seq).await;

            let outdir = outdir.join(seq.package().unwrap().name());
            fs::create_dir_all(&outdir).expect("failed to create output dir");

            fs::write(outdir.join(format!("{}.txt", seq.name())), plot).expect("failed to write output");
        }
    }
}

async fn plot_sequence(container: &Container, sequence: &ObjectRef) -> String {
    let mut plot = String::new();

    plot.push_str("@startuml\n");
    plot.push_str("hide empty description\n");
    plot.push_str(&plot_subsequence(container, sequence).await);
    plot.push_str("@enduml\n");

    plot
}

fn plot_subsequence<'a>(container: &'a Container, sequence: &'a ObjectRef) -> BoxFuture<'a, String> {
    async move {
        let mut plot = String::new();

        // define all states
        for obj in sequence.children() {
            let obj_name = format!("{}_{}", obj.parent().unwrap().name(), obj.name());

            if obj.class().name() == "Sequence" {
                plot.push_str(format!("state \"{}\" as {} {{\n",  obj.name(), obj_name).as_str());
                plot.push_str(&plot_subsequence(container, obj).await);
                plot.push_str("}\n");
            } else if obj.class().name().contains("SeqAct") 
                || obj.class().name().contains("SeqEvent") 
                || obj.class().name().contains("SeqCond") 
                || obj.class().name().contains("RUSequenceFunction")  
            {
                let seq = container.deserialize::<ScriptObject>(obj).await
                    .expect("deserialization failed");

                plot.push_str(format!("state \"{}\" as {} {{\n", obj.name(), obj_name).as_str());

                // add input links
                if let Some(ObjectProperty::Array(input_links)) = seq.attrib("InputLinks") {
                    for (idx, link) in input_links.iter().enumerate().filter_map(|(idx, a)| match &a {
                        ObjectProperty::Struct(_, link) => Some((idx, link)),
                        _ => None,
                    }) {
                        let desc = if let Some(ObjectProperty::String(desc)) = link.attrib("LinkDesc") {
                            desc.to_owned()
                        } else {
                            format!("Input{idx}")
                        };

                        plot.push_str(format!("state \"{desc}\" as {obj_name}_input{idx} <<inputPin>>\n", ).as_str());
                    }
                }

                // add output links
                if let Some(ObjectProperty::Array(output_links)) = seq.attrib("OutputLinks") {
                    for (idx, link) in output_links.iter().enumerate().filter_map(|(idx, a)| match &a {
                        ObjectProperty::Struct(_, link) => Some((idx, link)),
                        _ => None,
                    }) {
                        let desc = if let Some(ObjectProperty::String(desc)) = link.attrib("LinkDesc") {
                            desc.to_owned()
                        } else {
                            format!("Input{idx}")
                        };

                        plot.push_str(format!("state \"{desc}\" as {obj_name}_output{idx} <<outputPin>>\n").as_str());
                    }
                }

                plot.push_str("}\n");

                // Shared object properties
                if let Some(ObjectProperty::String(msg)) = seq.attrib("ObjComment") {
                    plot.push_str(format!("{obj_name} : {msg}\n").as_str());
                }

                if let Some(ObjectProperty::String(value)) = seq.attrib("ObjName") {
                    plot.push_str(format!("{obj_name} : ObjName: {value} \n").as_str());
                }

                if let Some(ObjectProperty::String(value)) = seq.attrib("EventDesc") {
                    plot.push_str(format!("{obj_name} : EventDesc: {value} \n").as_str());
                }

                // add additional data
                match obj.class().name() {
                    "SeqAct_Log" => {
                        if let Some(ObjectProperty::String(msg)) = seq.attrib("LogMessage") {
                            plot.push_str(format!("{obj_name} : LogMessage: {msg}\n").as_str());
                        }
                    },
                    "OLSeqEvent_Travel" => {
                        if let Some(ObjectProperty::String(name)) = seq.attrib("EventName") {
                            plot.push_str(format!("{obj_name} : EventName: {name}\n").as_str());
                        }
                    },
                    "OLSeqCond_Bypass" => {
                        if let Some(ObjectProperty::String(val)) = seq.attrib("BypassGroup") {
                            plot.push_str(format!("{obj_name} : BypassGroup: {val}\n").as_str());
                        }

                        if let Some(ObjectProperty::String(val)) = seq.attrib("BypassName") {
                            plot.push_str(format!("{obj_name} : BypassName: {val}\n").as_str());
                        }
                    },
                    "RUSeqAct_CallCinematic" => {
                        if let Some(ObjectProperty::String(msg)) = seq.attrib("CinematicProxyName") {
                            plot.push_str(format!("{obj_name} : CinematicProxyName: {msg}\n").as_str());
                        }
                    },
                    "RUSeqEvent_CinematicActivated" => {
                        if let Some(ObjectProperty::Name(msg)) = seq.attrib("CinematicName") {
                            plot.push_str(format!("{obj_name} : CinematicName: {msg}\n").as_str());
                        }
                    },
                    "OLSeqAct_SetServerState" => {
                        if let Some(ObjectProperty::Enum(name, value)) = seq.attrib("ConnectionState") {
                            plot.push_str(format!("{obj_name} : ConnectionState: {name} {:?}\n", value.as_ref()).as_str());
                        }
                    },
                    "OLSeqAct_WaitForPlayerConnectionState" => {
                        if let Some(ObjectProperty::Enum(name, value)) = seq.attrib("StateToWaitFor") {
                            plot.push_str(format!("{obj_name} : StateToWaitFor: {name} {:?}\n", value.as_ref()).as_str());
                        }
                    },
                    "SeqAct_ActivateRemoteEvent" => {
                        if let Some(ObjectProperty::Name(value)) = seq.attrib("EventName") {
                            plot.push_str(format!("{obj_name} : EventName: {value} \n").as_str());
                        }
                    },
                    "SeqEvent_RemoteEvent" => {
                        if let Some(ObjectProperty::Name(value)) = seq.attrib("EventName") {
                            plot.push_str(format!("{obj_name} : EventName: {value} \n").as_str());
                        }
                    },
                    "SeqEvent_Console" => {
                        if let Some(ObjectProperty::Name(value)) = seq.attrib("ConsoleEventName") {
                            plot.push_str(format!("{obj_name} : ConsoleEventName: {value} \n").as_str());
                        }
                    },
                    "SeqAct_ConsoleCommand" => {
                        if let Some(ObjectProperty::Array(args)) = seq.attrib("Commands") {
                            let cmd_args = args.iter().filter_map(|arg| match arg {
                                ObjectProperty::String(str) => Some(str.to_owned()),
                                ObjectProperty::Name(name) => Some(name.to_string()),
                                _ => None,
                            }).collect::<Vec<_>>();

                            plot.push_str(format!("{obj_name} : Commands: {} \n", cmd_args.join(" ")).as_str());
                        }
                    },
                    "SeqAct_ControlGameMovie" => {
                        if let Some(ObjectProperty::String(name)) = seq.attrib("MovieName") {
                            plot.push_str(format!("{obj_name} : MovieName: {name}\n").as_str());
                        }
                    },
                    "SeqAct_RequestServerAction" => {
                        if let Some(ObjectProperty::String(name)) = seq.attrib("SequenceName") {
                            plot.push_str(format!("{obj_name} : SequenceName: {name}\n").as_str());
                        }
                    },
                    "OLSeqEvent_Cinematic" => {
                        if let Some(ObjectProperty::String(name)) = seq.attrib("EventName") {
                            plot.push_str(format!("{obj_name} : EventName: {name}\n").as_str());
                        }
                        if let Some(ObjectProperty::String(name)) = seq.attrib("LevelName") {
                            plot.push_str(format!("{obj_name} : LevelName: {name}\n").as_str());
                        }
                        if let Some(ObjectProperty::String(name)) = seq.attrib("CinematicName") {
                            plot.push_str(format!("{obj_name} : CinematicName: {name}\n").as_str());
                        }
                    },
                    _ => {},
                }
            }
        }

        // link states
        for obj in sequence.children() {
            let obj_name = format!("{}_{}", obj.parent().unwrap().name(), obj.name());

            if obj.class().name().contains("SeqAct") 
                || obj.class().name().contains("SeqEvent") 
                || obj.class().name().contains("SeqCond")
                || obj.class().name().contains("RUSequenceFunction")
            {
                let seq = container.deserialize::<ScriptObject>(obj).await
                    .expect("deserialization failed");

                // add input links
                if let Some(ObjectProperty::Array(input_links)) = seq.attrib("InputLinks") {
                    for (input_idx, link) in input_links.iter().enumerate().filter_map(|(idx, a)| match &a {
                        ObjectProperty::Struct(_, link) => Some((idx, link)),
                        _ => None,
                    }) {
                        if let Some(ObjectProperty::Array(links)) = link.attrib("Links") {
                            let do_fork = links.len() > 1;

                            if do_fork {
                                plot.push_str(format!("state {obj_name}_join_i{input_idx} <<join>>\n").as_str());
                                plot.push_str(format!("{obj_name}_join_i{input_idx} ---> {obj_name}_input{input_idx}\n").as_str());
                            }

                            for link in links.iter().filter_map(|a| match &a {
                                ObjectProperty::Struct(_, link) => Some(link),
                                _ => None,
                            }) {
                                let output_idx = if let Some(ObjectProperty::Int(idx)) = link.attrib("InputLinkIdx") {
                                    idx
                                } else {
                                    panic!("Link without InputLinkIdx");
                                };

                                let output_obj = if let Some(ObjectProperty::Object(output_obj)) = link.attrib("LinkedOp") {
                                    output_obj
                                } else {
                                    panic!("Link without LinkedOp");
                                };

                                let output_obj_name = format!("{}_{}", output_obj.parent().unwrap().name(), output_obj.name());

                                if do_fork {
                                    plot.push_str(format!("{output_obj_name}_output{output_idx} -> {obj_name}_join_i{input_idx}\n").as_str());
                                } else {
                                    plot.push_str(format!("{output_obj_name}_output{output_idx} ---> {obj_name}_input{input_idx}\n").as_str());
                                }
                            }
                        }

                    }
                }

                // add output links
                if let Some(ObjectProperty::Array(output_links)) = seq.attrib("OutputLinks") {
                    for (output_idx, link) in output_links.iter().enumerate().filter_map(|(idx, a)| match &a {
                        ObjectProperty::Struct(_, link) => Some((idx, link)),
                        _ => None,
                    }) {
                        if let Some(ObjectProperty::Array(links)) = link.attrib("Links") {
                            let do_fork = links.len() > 1;

                            if do_fork {
                                plot.push_str(format!("state {obj_name}_fork_o{output_idx} <<fork>>\n").as_str());
                                plot.push_str(format!("{obj_name}_output{output_idx} --> {obj_name}_fork_o{output_idx}\n").as_str());
                            }

                            for link in links.iter().filter_map(|a| match &a {
                                ObjectProperty::Struct(_, link) => Some(link),
                                _ => None,
                            }) {
                                let input_idx = if let Some(ObjectProperty::Int(idx)) = link.attrib("InputLinkIdx") {
                                    idx
                                } else {
                                    panic!("Link without InputLinkIdx");
                                };

                                if let Some(ObjectProperty::Object(input_obj)) = link.attrib("LinkedOp") {
                                    let input_obj_name = format!("{}_{}", input_obj.parent().unwrap().name(), input_obj.name());
                            
                                    if do_fork {
                                        plot.push_str(format!("{obj_name}_fork_o{output_idx} ---> {input_obj_name}_input{input_idx}\n").as_str());
                                    } else {
                                        plot.push_str(format!("{obj_name}_output{output_idx} ---> {input_obj_name}_input{input_idx}\n").as_str());
                                    }
                                };
                            }
                        }

                    }
                }

                // add variable links
                if let Some(ObjectProperty::Array(variable_links)) = seq.attrib("VariableLinks") {
                    for (_, link) in variable_links.iter().enumerate().filter_map(|(idx, a)| match &a {
                        ObjectProperty::Struct(_, link) => Some((idx, link)),
                        _ => None,
                    }) {
                        if let Some(ObjectProperty::Bool(false)) = link.attrib("bHidden") {

                            plot.push_str(format!("note left of {obj_name}\n").as_str());

                            if let Some(ObjectProperty::String(desc)) = link.attrib("LinkDesc") {
                                plot.push_str(format!("  Desc: {desc}\n").as_str());
                            };

                            if let Some(ObjectProperty::Name(name)) = link.attrib("PropertyName") {
                                plot.push_str(format!("  Property: {name}\n").as_str());
                            };

                            if let Some(ObjectProperty::Array(vars)) = link.attrib("LinkedVariables") {
                                if let Some(ObjectProperty::Object(obj)) = vars.first() {
                                    let var = container.deserialize::<ScriptObject>(obj).await
                                        .expect("deserialization failed");

                                    plot.push_str(format!("  LinkedVar: {}\n", obj.name()).as_str());

                                    if let Some(ObjectProperty::Name(name)) = var.attrib("FindVarName") {
                                        plot.push_str(format!("  FindVarName: {name}\n").as_str());
                                    }

                                    if let Some(ObjectProperty::Name(name)) = var.attrib("VarName") {
                                        plot.push_str(format!("  VarName: {name}\n").as_str());
                                    }

                                    if let Some(ObjectProperty::String(val)) = var.attrib("StrValue") {
                                        plot.push_str(format!("  StrValue: {val}\n").as_str());
                                    }

                                    if let Some(ObjectProperty::Int(val)) = var.attrib("IntValue") {
                                        plot.push_str(format!("  IntValue: {val}\n").as_str());
                                    }

                                    if let Some(ObjectProperty::Float(val)) = var.attrib("FloatValue") {
                                        plot.push_str(format!("  FloatValue: {val}\n").as_str());
                                    }

                                    if let Some(ObjectProperty::Object(val)) = var.attrib("ObjValue") {
                                        plot.push_str(format!("  ObjValue: {}\n", val.name()).as_str());
                                    }
                                }
                            };

                            plot.push_str("end note\n");
                        }
                    }
                }
            }
        }

        plot
    }.boxed()
}
