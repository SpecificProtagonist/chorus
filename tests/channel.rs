mod common;
use chorus::types::Channel;

#[tokio::test]
async fn get_channel() {
    let mut bundle = common::setup().await;
    let bundle_channel = bundle.channel.clone();
    let bundle_user = &mut bundle.user;

    assert_eq!(
        bundle_channel,
        Channel::get(
            bundle_user.token.as_str(),
            bundle.instance.urls.get_api(),
            &bundle_channel.id.to_string(),
            &mut bundle_user.limits,
            &mut bundle.instance.limits
        )
        .await
        .unwrap()
    );
    common::teardown(bundle).await
}
