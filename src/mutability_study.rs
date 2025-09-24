pub fn mutability_study_fn(){
    // Understanding immutability in Rust
    // Chapter 1: immutable vars
    let immutable_var = 0; // this value is immutable. It can never be changed. It is also owned in this scope.
    // Uncommenting the following line will trigger a compile time error. immutable_var is not assignable.
    // immutable_var = 1;

    // Chapter 2: mutable vars
    let mut mutable_var = 0; // this value is mutable.
    println!("mutable_var (pre mutation by direct access): {}", mutable_var);
    mutable_var = 2;
    println!("mutable_var (post mutation by direct access): {}", mutable_var);

    // Chapter 3: immutable references to mutable vars
    // Mutability of mutable_var: currently not allowed, as there is an immutable reference, immutable_reference_to_mutable_var.
    let immutable_reference_to_mutable_var = &mutable_var; 
    println!("mutable_var (by immutable reference): {}", mutable_var);
    // Uncommenting the following line will trigger a compile time error. immutable reference is not assignable
    // *immutable_reference_to_mutable_var = 3;

    // Chapter 4: immutable binding to mutable reference to mutable vars
    {
        // Mutability of mutable_var: currently borrowed by immutable_binding_to_mutable_reference_to_mutable_var.
        // This might seem illegal due to immutable_reference_to_mutable_var. However, it is not, because the Rust compiler isn't using only scope to determine lifetime.
        // Since there are no further references to immutable_reference_to_mutable_var, the Rust compiler considers its monopoly on the mutability of mutable_var to be over.
        let immutable_binding_to_mutable_reference_to_mutable_var = &mut mutable_var; 
        // Uncommenting the following line will trigger a compile time error. Mutability rights are exclusive, and the existence of immutable_binding_to_mutable_reference_to_mutable_var means that mutable_var can not be directly used.
        // println!("mutable_var (pre mutation by mutable reference): {}", mutable_var);
        println!("mutable_var (pre mutation by mutable reference): {}", *immutable_binding_to_mutable_reference_to_mutable_var);
        *immutable_binding_to_mutable_reference_to_mutable_var = 4;
        println!("mutable_var (post mutation by mutable reference): {}", *immutable_binding_to_mutable_reference_to_mutable_var);
    }

    // Chapter 5: mutable binding to mutable references to mutable vars
    {
        let mut mutable_binding_to_mutable_reference_to_mutable_var = &mut mutable_var;
        let mut new_mutable_var = 5;
        // Uncommenting the following line will trigger a compile time error. Mutability rights are exclusive, and the existence of mutable_binding_to_mutable_reference_to_mutable_var means that mutable_var can not be directly used.
        // println!("*mutable_binding (pre mutation by mutable binding - cannot directly access mutable_var): {}", mutable_var);
        println!("*mutable_binding (pre mutation by mutable binding): {}", *mutable_binding_to_mutable_reference_to_mutable_var);
        

        mutable_binding_to_mutable_reference_to_mutable_var = &mut new_mutable_var;
        println!("*mutable_binding (post mutation by mutable binding - can now directly access mutable_var): {}", mutable_var);
        println!("*mutable_binding (post mutation by mutable binding): {}", *mutable_binding_to_mutable_reference_to_mutable_var);
    }

    // Chapter 6: mutable binding to immutable references to mutable vars
    let mut another_mutable_var = 6;
    let immutable_reference_to_another_mutable_var = &another_mutable_var;
    // Mutability of mutable_var:
    // Uncommenting the following line will error. Why?
    // let mut mutable_binding_to_immutable_reference_to_mutable_var = immutable_reference_to_mutable_var;
    // Adding the above line in this locatino would extend the lifetime of the exclusivity of immutable_reference_to_mutable_var
    let mut mutable_binding_to_immutable_reference_to_mutable_var = immutable_reference_to_another_mutable_var;
    println!("*mutable_binding (pre mutation by mutable binding to immutable reference): {}", *mutable_binding_to_immutable_reference_to_mutable_var);
    
    let mut one_more_mutable_var = 7;
    let immutable_reference_to_one_more_mutable_var = &one_more_mutable_var;
    mutable_binding_to_immutable_reference_to_mutable_var = immutable_reference_to_one_more_mutable_var;
    println!("*mutable_binding (post mutation by mutable binding to immutable reference): {}", *mutable_binding_to_immutable_reference_to_mutable_var);
}