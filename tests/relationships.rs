use chorus::types::{self, RegisterSchema, Relationship, RelationshipType};

mod common;

#[tokio::test]
async fn test_get_mutual_relationships() {
    let register_schema = RegisterSchema {
        username: "integrationtestuser2".to_string(),
        consent: true,
        date_of_birth: Some("2000-01-01".to_string()),
        ..Default::default()
    };

    let mut bundle = common::setup().await;
    let belongs_to = &mut bundle.instance;
    let user = &mut bundle.user;
    let mut other_user = belongs_to.register_account(&register_schema).await.unwrap();
    let friend_request_schema = types::FriendRequestSendSchema {
        username: user.object.username.clone(),
        discriminator: Some(user.object.discriminator.clone()),
    };
    other_user.send_friend_request(friend_request_schema).await;
    let relationships = user
        .get_mutual_relationships(other_user.object.id)
        .await
        .unwrap();
    println!("{:?}", relationships);
    common::teardown(bundle).await
}

#[tokio::test]
async fn test_get_relationships() {
    let register_schema = RegisterSchema {
        username: "integrationtestuser2".to_string(),
        consent: true,
        date_of_birth: Some("2000-01-01".to_string()),
        ..Default::default()
    };

    let mut bundle = common::setup().await;
    let belongs_to = &mut bundle.instance;
    let user = &mut bundle.user;
    let mut other_user = belongs_to.register_account(&register_schema).await.unwrap();
    let friend_request_schema = types::FriendRequestSendSchema {
        username: user.object.username.clone(),
        discriminator: Some(user.object.discriminator.clone()),
    };
    other_user.send_friend_request(friend_request_schema).await;
    let relationships = user.get_relationships().await.unwrap();
    assert_eq!(relationships.get(0).unwrap().id, other_user.object.id);
    common::teardown(bundle).await
}

#[tokio::test]
async fn test_modify_relationship_friends() {
    let register_schema = RegisterSchema {
        username: "integrationtestuser2".to_string(),
        consent: true,
        date_of_birth: Some("2000-01-01".to_string()),
        ..Default::default()
    };

    let mut bundle = common::setup().await;
    let belongs_to = &mut bundle.instance;
    let user = &mut bundle.user;
    let mut other_user = belongs_to.register_account(&register_schema).await.unwrap();
    other_user
        .modify_user_relationship(user.object.id, types::RelationshipType::Friends)
        .await;
    let relationships = user.get_relationships().await.unwrap();
    assert_eq!(relationships.get(0).unwrap().id, other_user.object.id);
    assert_eq!(
        relationships.get(0).unwrap().relationship_type,
        RelationshipType::Incoming
    );
    let relationships = other_user.get_relationships().await.unwrap();
    assert_eq!(relationships.get(0).unwrap().id, user.object.id);
    assert_eq!(
        relationships.get(0).unwrap().relationship_type,
        RelationshipType::Outgoing
    );
    user.modify_user_relationship(other_user.object.id, RelationshipType::Friends)
        .await;
    assert_eq!(
        other_user
            .get_relationships()
            .await
            .unwrap()
            .get(0)
            .unwrap()
            .relationship_type,
        RelationshipType::Friends
    );
    user.remove_relationship(other_user.object.id).await;
    assert_eq!(
        other_user.get_relationships().await.unwrap(),
        Vec::<Relationship>::new()
    );
    common::teardown(bundle).await
}

#[tokio::test]
async fn test_modify_relationship_block() {
    let register_schema = RegisterSchema {
        username: "integrationtestuser2".to_string(),
        consent: true,
        date_of_birth: Some("2000-01-01".to_string()),
        ..Default::default()
    };

    let mut bundle = common::setup().await;
    let belongs_to = &mut bundle.instance;
    let user = &mut bundle.user;
    let mut other_user = belongs_to.register_account(&register_schema).await.unwrap();
    other_user
        .modify_user_relationship(user.object.id, types::RelationshipType::Blocked)
        .await;
    let relationships = user.get_relationships().await.unwrap();
    assert_eq!(relationships, Vec::<Relationship>::new());
    let relationships = other_user.get_relationships().await.unwrap();
    assert_eq!(relationships.get(0).unwrap().id, user.object.id);
    assert_eq!(
        relationships.get(0).unwrap().relationship_type,
        RelationshipType::Blocked
    );
    other_user.remove_relationship(user.object.id).await;
    assert_eq!(
        other_user.get_relationships().await.unwrap(),
        Vec::<Relationship>::new()
    );
    common::teardown(bundle).await
}
