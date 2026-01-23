use bbean_network::peer::{PeerInfo, PeerManager};
use bbean_network::protocol::{MessageType, ProtocolMessage};

#[tokio::test]
async fn test_peer_add_remove() {
    let manager = PeerManager::new(10);
    let peer = PeerInfo {
        id: "peer-1".into(),
        address: "127.0.0.1:9000".parse().unwrap(),
        user_agent: "bbean-test/0.1".into(),
        capabilities: vec!["webgpu".into()],
        connected_at: chrono::Utc::now(),
        last_seen: chrono::Utc::now(),
        latency_ms: None,
    };

    manager.add_peer(peer).await.unwrap();
    assert_eq!(manager.peer_count().await, 1);

    let retrieved = manager.get_peer("peer-1").await.unwrap();
    assert_eq!(retrieved.id, "peer-1");

    manager.remove_peer("peer-1").await;
    assert_eq!(manager.peer_count().await, 0);
}

#[tokio::test]
async fn test_peer_capacity() {
    let manager = PeerManager::new(2);
    for i in 0..2 {
        let peer = PeerInfo {
            id: format!("peer-{}", i),
            address: format!("127.0.0.1:{}", 9000 + i).parse().unwrap(),
            user_agent: "test".into(),
            capabilities: vec![],