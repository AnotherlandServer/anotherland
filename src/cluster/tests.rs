use std::{fmt::Arguments, marker::PhantomData};

use actor_macros::actor_actions;
use async_trait::async_trait;

use crate::cluster::actor::ClusterNode;

use super::actor::{Actor, ActorResult};

/*struct TestActor;

pub struct TestResult;

#[async_trait]
impl Actor for TestActor {
    async fn pre_start(&mut self) -> ActorResult<()> { 
        println!("pre_start");
        Ok(()) 
    }
    async fn started(&mut self) -> ActorResult<()> { 
        println!("started");
        Ok(()) 
    }
    async fn stopped(&mut self) -> ActorResult<()> { 
        println!("stopped");
        Ok(()) 
    }
}

#[actor_actions]
impl TestActor {
    pub async fn test_mut_async_action(&mut self) -> ActorResult<TestResult> {
        println!("test_mut_async_action");
        Ok(TestResult)
    }

    pub fn test_mut_action(&mut self) {
        println!("test_mut_action");
    }

    pub fn test_action(&self) {
        println!("test_action");
    }

    pub fn test_action_args(&self, a: String, b: i32) -> ActorResult<TestResult> {
        println!("test_action_args ({}, {})", a, b);
        Ok(TestResult)
    }

    pub async fn test_action_async_args(&self, a: String, b: i32) -> ActorResult<TestResult> {
        println!("test_action_async_args ({}, {})", a, b);
        Ok(TestResult)
    }
}

#[tokio::test]
async fn action_test() {
    let test_actor = TestActor {};
    let mut node = ClusterNode::new();

    node.add_actor("test_actor", test_actor);

    let mut test_actor = node.get_actor::<TestActor>("test_actor").expect("actor not found");
    test_actor.test_mut_async_action().await.unwrap();
    test_actor.test_mut_action().await;
    test_actor.test_action().await;
    test_actor.test_action_args("test1".to_owned(), 1234).await.unwrap();
    test_actor.test_action_async_args("test2".to_owned(), 5678).await.unwrap();
}*/
