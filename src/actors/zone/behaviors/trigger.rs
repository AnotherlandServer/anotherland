/*
EntityType::Trigger => {
    if cmd_args[0] == "triggeraction" {
        let trigger = self.app.world.get::<TriggerClass>(*target_entity).unwrap();

        if let Some(script) = trigger.lua_script() {
            match script.deref() {
                "ClassClear" => {
                    let mut player = self.app.world.get_mut::<PlayerClass>(*player_entity).unwrap();

                    let mut update = ParamSet::new();
                    update.insert(PlayerAttribute::CombatStyle, 6);

                    player.apply(update.clone());

                    let event_sender = self.event_sender.clone();

                    tokio::spawn(async move {
                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                            avatar_id: instigator, 
                            params: update.into_box(),
                        }));
                    });
                },
                "ClassEnergizer" => {
                    let mut player = self.app.world.get_mut::<PlayerClass>(*player_entity).unwrap();

                    let mut update = ParamSet::new();
                    update.insert(PlayerAttribute::CombatStyle, 3);

                    player.apply(update.clone());

                    let event_sender = self.event_sender.clone();

                    tokio::spawn(async move {
                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                            avatar_id: instigator, 
                            params: update.into_box(),
                        }));
                    });
                },
                "ClassWarrior" => {
                    let mut player = self.app.world.get_mut::<PlayerClass>(*player_entity).unwrap();

                    let mut update = ParamSet::new();
                    update.insert(PlayerAttribute::CombatStyle, 0);

                    player.apply(update.clone());

                    let event_sender = self.event_sender.clone();

                    tokio::spawn(async move {
                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                            avatar_id: instigator, 
                            params: update.into_box(),
                        }));
                    });
                },
                "ClassMarksman" => {
                    let mut player = self.app.world.get_mut::<PlayerClass>(*player_entity).unwrap();

                    let mut update = ParamSet::new();
                    update.insert(PlayerAttribute::CombatStyle, 1);

                    player.apply(update.clone());

                    let event_sender = self.event_sender.clone();

                    tokio::spawn(async move {
                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                            avatar_id: instigator, 
                            params: update.into_box(),
                        }));
                    });
                },
                "ClassAssassin" => {
                    let mut player = self.app.world.get_mut::<PlayerClass>(*player_entity).unwrap();

                    let mut update = ParamSet::new();
                    update.insert(PlayerAttribute::CombatStyle, 2);

                    player.apply(update.clone());

                    let event_sender = self.event_sender.clone();

                    tokio::spawn(async move {
                        let _ = event_sender.send(Arc::new(ZoneEvent::AvatarUpdated { 
                            avatar_id: instigator, 
                            params: update.into_box(),
                        }));
                    });
                },
                _ => warn!("Unimplemented lua script '{}' for trigger", script),
            }
        };
    } else {
        warn!("Unimplemented behavior '{}' for trigger", behavior);
    }
*/