#[cfg(test)]
mod tests{
use super::*;
use deadlock_detect::DeadlockDetector;

#[test]
fn test_deadlock_detection() {
    let mut detector = DeadlockDetector::new();

    detector.add_process("procA")
    detector.add_process("procB");
    detector.add_resource("resC");
    detector.add_resource("resD");

    assert!(detector.request("procA", "resC"));
    assert!(detector.request("procB", "resD"));
    assert!(detector.request("procA", "resD"));
    assert!(detector.request("procB", "resC"));
}
}