#[cfg(test)]
mod tests{
use super::*;
use deadlock_detect::DeadlockDetector;

	//Creates a cycle through:
	//A->D->B->C->A
    #[test]
	fn two_procs_two_res_deadlock() {
	let mut detector = DeadlockDetector::new();

	detector.add_process("procA");
	detector.add_process("procB");
	detector.add_resource("resC");
	detector.add_resource("resD");

	let result = detector.request("procA","resC"); //OK
	assert!(result); //should be true
	let result = detector.request("procB","resD"); //OK
	assert!(result); //should be true
	let result = detector.request("procA","resD"); //OK
	assert!(result); //should be true
	let result = detector.request("procB","resC"); //Deadlock
	assert!(!result); //result should be false
	}

	//Does not create a cycle
    #[test]
	fn two_procs_two_res_no_deadlock() {
	let mut detector = DeadlockDetector::new();

	detector.add_process("procA");
	detector.add_process("procB");
	detector.add_resource("resC");
	detector.add_resource("resD");

	let result = detector.request("procA","resC"); //OK
	assert!(result); //should be true
	let result = detector.request("procB","resD"); //OK
	assert!(result); //should be true
	let result = detector.request("procA","resD"); //OK
	assert!(result); //should be true
	}

	//Processes may wait on more than one resource at once
	//No deadlock
	#[test]
	fn multiple_wait() {
	let mut detector = DeadlockDetector::new();

	detector.add_process("procA");
	detector.add_resource("resB");
	detector.add_resource("resC");
	detector.add_resource("resD");

	let result = detector.request("procA","resB"); //OK
	assert!(result); //should be true
	let result = detector.request("procA","resC"); //OK
	assert!(result); //should be true
	let result = detector.request("procA","resD"); //OK
	assert!(result); //should be true
	}

	//Creates a cycle through:
	//A->C->A
	#[test]
	fn self_deadlock() {
	let mut detector = DeadlockDetector::new();

	detector.add_process("procA");
	detector.add_resource("resC");
	let result = detector.request("procA","resC"); //OK
	assert!(result); //should be true
	let result = detector.request("procA","resC"); //OK
	assert!(!result); //result should be false
	}

	//Creates TWO cycles through:
	//A->D->B->C->A 
	//and
	//A->F->B->C->A
	//
	//Your code only has to find one and return false
    #[test]
	fn two_cycles() {
	let mut detector = DeadlockDetector::new();

	detector.add_process("procA");
	detector.add_process("procB");
	detector.add_resource("resC");
	detector.add_resource("resD");
	detector.add_resource("resE");
	detector.add_resource("resF");

	let result = detector.request("procA","resC"); //OK
	assert!(result); //should be true
	let result = detector.request("procB","resD"); //OK
	assert!(result); //should be true
	let result = detector.request("procA","resE"); //OK
	assert!(result); //should be true
	let result = detector.request("procB","resF"); //OK
	assert!(result); //should be true
	let result = detector.request("procA","resD"); //OK
	assert!(result); //should bie true
	let result = detector.request("procA","resF"); //OK
	assert!(result); //should bie true
	let result = detector.request("procB","resC"); //Deadlock
	assert!(!result); //result should be false	
	}

	//Creates a cycle through:
	// A->F->B->G->C->H->D->I->E->J->A
	#[test]
	fn big_loop(){
	let mut detector = DeadlockDetector::new();

	detector.add_process("procA");
	detector.add_process("procB");
	detector.add_process("procC");
	detector.add_process("procD");
	detector.add_process("procE");
	detector.add_resource("resF");
	detector.add_resource("resG");
	detector.add_resource("resH");
	detector.add_resource("resI");
	detector.add_resource("resJ");
	let result = detector.request("procA","resJ"); //OK
	assert!(result); //should be true
	let result = detector.request("procB","resF"); //OK
	assert!(result); //should be true
	let result = detector.request("procC","resG"); //OK
	assert!(result); //should be true
	let result = detector.request("procD","resH"); //OK
	assert!(result); //should be true
	let result = detector.request("procE","resI"); //OK
	assert!(result); //should be true
	let result = detector.request("procB","resG"); //OK
	assert!(result); //should be true
	let result = detector.request("procC","resH"); //OK
	assert!(result); //should be true
	let result = detector.request("procD","resI"); //OK
	assert!(result); //should be true
	let result = detector.request("procE","resJ"); //OK
	assert!(result); //should be true
	let result = detector.request("procA","resF"); //OK
	assert!(!result); //result should be false - deadlock
	}

	//Contains a cycle through:
	//A->B->D->F->I->M->A
	// Note that you can draw this structure sensibly
	// by looking at X level of tree below.
	#[test]
	fn tree_structure(){
	let mut detector = DeadlockDetector::new();

	//First level of tree
	detector.add_resource("resA");
	
	//Second level of tree
	detector.add_process("procB");

	//Third level of tree
	detector.add_resource("resC");
	detector.add_resource("resD");

	//Fourth level of tree
	detector.add_process("procE");
	detector.add_process("procF");

	//Fifth level of tree
	detector.add_resource("resG");
	detector.add_resource("resH");
	detector.add_resource("resI");
	detector.add_resource("resJ");

	//Sixth level of tree

	detector.add_process("procK");
	detector.add_process("procL");
	detector.add_process("procM");
	detector.add_process("procN");
	
	//Level one to two
	assert!( detector.request("procB","resA") ); //OK

	//Level three to four
	assert!( detector.request("procE","resC") ); //OK
	assert!( detector.request("procF","resD") ); //OK

	//Level five to six
	assert!( detector.request("procK","resG") ); //OK
	assert!( detector.request("procL","resH") ); //OK
	assert!( detector.request("procM","resI") ); //OK
	assert!( detector.request("procN","resJ") ); //OK

	//Level two to three
	assert!( detector.request("procB","resC") ); //OK
	assert!( detector.request("procB","resD") ); //OK

	//Level four to five
	assert!( detector.request("procE","resG") ); //OK
	assert!( detector.request("procE","resH") ); //OK
	assert!( detector.request("procF","resI") ); //OK
	assert!( detector.request("procF","resJ") ); //OK

	//Level six back to one- causes deadlock
	assert!( !detector.request("procM","resA") );

	}

	//NOTE: This is the same tree as above but builds it
	// in reverse order.
	// 
	// Contains a cycle through:
	// A->B->D->F->I->M->A
	//
	// Note that you can draw this structure sensibly
	// by looking at X level of tree below.
	#[test]
	fn tree_structure_reversed(){
	let mut detector = DeadlockDetector::new();

	//First level of tree
	detector.add_resource("resA");
	
	//Second level of tree
	detector.add_process("procB");

	//Third level of tree
	detector.add_resource("resC");
	detector.add_resource("resD");

	//Fourth level of tree
	detector.add_process("procE");
	detector.add_process("procF");

	//Fifth level of tree
	detector.add_resource("resG");
	detector.add_resource("resH");
	detector.add_resource("resI");
	detector.add_resource("resJ");

	//Sixth level of tree

	detector.add_process("procK");
	detector.add_process("procL");
	detector.add_process("procM");
	detector.add_process("procN");
	
	//Level five to six
	assert!( detector.request("procK","resG") ); //OK
	assert!( detector.request("procL","resH") ); //OK
	assert!( detector.request("procM","resI") ); //OK
	assert!( detector.request("procN","resJ") ); //OK

	//Level four to five
	assert!( detector.request("procE","resG") ); //OK
	assert!( detector.request("procE","resH") ); //OK
	assert!( detector.request("procF","resI") ); //OK
	assert!( detector.request("procF","resJ") ); //OK

	//Level three to four
	assert!( detector.request("procE","resC") ); //OK
	assert!( detector.request("procF","resD") ); //OK

	//Level two to three
	assert!( detector.request("procB","resC") ); //OK
	assert!( detector.request("procB","resD") ); //OK

	//Level one to two
	assert!( detector.request("procB","resA") ); //OK

	//Level six back to one- causes deadlock
	assert!( !detector.request("procM","resA") );

	}

    //This test just calls release, no deadlock expected
    #[test]
    fn call_release() {
    let mut detector = DeadlockDetector::new();

    detector.add_resource("resA");
    detector.add_process("procB");

    assert!( detector.request("procB", "resA") );
    assert!( detector.release("procB", "resA", None) );
    }

    //This test does not create deadlock if release actually
    //releases the resource, but will create deadlock if it
    //does not.
    #[test]
    fn no_deadlock_after_release() {
    let mut detector = DeadlockDetector::new();

    detector.add_resource("resA");
    detector.add_process("procB");

    assert!( detector.request("procB", "resA") );
    assert!( detector.release("procB", "resA", None) );
    //The next call would self deadlock B->A->B if we didn't
    //successfully release B->A in the last call.
    assert!( detector.request("procB", "resA") );
    assert!( !detector.can_deadlock("procB") );
    }

    //This test creates a deadlock only when releasing.
    //Deadlock through: A->E->B->D->A
    #[test]
    fn deadlock_from_release() {
    let mut detector = DeadlockDetector::new();

    detector.add_process("procA");
    detector.add_process("procB");
    detector.add_process("procC");
    detector.add_resource("resD");
    detector.add_resource("resE");

    assert!( detector.request("procC", "resE") );
    assert!( detector.request("procA", "resD") );
    assert!( detector.request("procA", "resE") );
    assert!( detector.request("procB", "resD") );
    assert!( detector.request("procB", "resE") );

    //The following call creates deadlock by switching
    //the direction of the edge from B->E to E->B
    assert!( !detector.release("procC", "resE", Some( "procB" )));
    }

    //This test does not create a deadlock through releasing, but it would
    //if next_process in the release call was procB
    #[test]
    fn almost_deadlock_from_release() {
    let mut detector = DeadlockDetector::new();

    detector.add_process("procA");
    detector.add_process("procB");
    detector.add_process("procC");
    detector.add_resource("resD");
    detector.add_resource("resE");

    assert!( detector.request("procC", "resE") );
    assert!( detector.request("procA", "resD") );
    assert!( detector.request("procA", "resE") );
    assert!( detector.request("procB", "resD") );
    assert!( detector.request("procB", "resE") );

    assert!( detector.release("procC", "resE", Some( "procA" )));
    }

    //Just calls release multiple times
    #[test]
    fn multiple_release() {
    let mut detector = DeadlockDetector::new();

    detector.add_process("procA");
    detector.add_process("procB");
    detector.add_process("procC");
    detector.add_resource("resD");

    assert!( detector.request("procA", "resD") );
    assert!( detector.request("procB", "resD") ); //waiting
    assert!( detector.request("procC", "resD") ); //waiting

    assert!( detector.release("procA", "resD", Some( "procB" )));
    assert!( detector.release("procB", "resD", Some( "procC" )));
    assert!( detector.release("procC", "resD", None));

    //Graph should have no edges at the end
    }
}