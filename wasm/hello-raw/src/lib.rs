use std::alloc::{alloc, dealloc, Layout};
use std::mem;


// Expose the function greet which takes a &str as input and returns a new heap allocated String:
//
// Note that we make it public and specify the extern "C" modifier to ensure it has the right calling convention.
//
// However we did not specify #[no_mangle] because we are not going to call this directly
// and therefore we want Rust to mangle the name so that the wrapper that we do expose can use the name greet.
pub extern "C" fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

// Webassembly does not understand &str or String types, it can just take 0 or more numeric inputs  and return a single numeric output.
// So we have to figure out how to turn &str into 1 or more numeric inputs and String into a single numeric output.
// There are actually many different ways to do this, each with trade-offs, and we are going to demonstrate one approach.
//
// We have turned the string input into a pointer (which is just an integer) to the beginning of where the string lives in memory
// and the length of the string (also an integer).
//
// Pointers and FFI (Foreign Function Interface)
// Pointers are not common in most of Rust because you usually use references when sharing data.
// However, when working across FFI (Foreign Function Interface) boundaries, e.g. when talking to JavaScript,
// we use pointers directly as they are a concept shared on both sides.
//
// We take a pointer to a memory region containing bytes (a u8 value is just one byte) which represents a string  and the length of that string.
// It is the responsibility of the caller to ensure that this pointer is valid and that the length is correct.
//
// The return type is a pointer to a mutable String, i.e. a heap allocated String.
// Recall that String itself is already on the heap so this is an extra allocation.
// It is an implicit contract of our function that the caller is responsible for making sure that this pointer
// gets passed back to Rust at some point to clean up the memory on the heap.
// By returning a mutable pointer we are effectively saying you own this thing that we created
// because mutability in Rust comes from being the sole owner of an object.
// This is the same contract as our original greet function as the caller becomes the owner of the returned String.
// This concept is just a bit more implicit due to the FFI boundary.
#[export_name = "greet"]
pub extern "C" fn __greet_wrapper(
    arg0_ptr: *const u8,
    arg0_len: usize,
) -> *mut String {
    // Turn pointer into &str to pass to original greet function
    //
    // This is inherently an unsafe operation as the compiler has no mechanism to verify that the
    // pointer actually points to useful data or that the length is correct.
    // We use an unsafe block to tell the compiler we know it cannot check this for us but we are guaranteeing that the
    // relevant invariants are satisfied. The standard library provides a function std::slice::from_raw_parts which will give us a &[u8].
    // This function is marked as unsafe which is why we have to call it inside an unsafe block.
    //
    // This is the canonical way to use unsafe code in Rust. Keep the block as small as possible to get back into safe Rust quickly.
    let arg0 = unsafe {
        let slice = std::slice::from_raw_parts(arg0_ptr, arg0_len);
        std::str::from_utf8_unchecked(slice)
    };
    let _ret = greet(arg0);
    // Turn String returned from greet into a pointer which again is just an integer.
    //
    // We then need to somehow give a pointer to this String to the caller and ensure that the underlying memory is not deallocated.
    //
    // We use Box::new to create a new heap allocation to hold the String.
    //
    // Note that the String is already on the heap by its nature, but we create a new heap allocation to own the String
    // because we can keep the String alive if we can keep the Box alive as it becomes the owner of the String.
    // We do this by using the associated function Box::into_raw.
    // This function consumes the Box and returns exactly the raw pointer we want.
    //
    // The documentation for Box::into_raw is clear that whoever gets this raw pointer is responsible for
    // managing the memory and must do something to ensure the box gets destroyed when it is no longer needed, otherwise you will leak memory.
    Box::into_raw(Box::new(_ret))
}

// First the signature should be expected by now:
// - we want this publicly accessible
// - we want it to use the “C” calling convention
// - we want the name to be __malloc so we specify #[no_mangle].
// We take a size as input (we use the byte length of our string)
// and return a pointer to some allocated bytes of that size.
#[no_mangle]
pub extern "C" fn __malloc(size: usize) -> *mut u8 {
    // The first thing we do is get the minimum alignment for a usize based on the ABI.
    // We need this to pass to the Layout constructor because in order to allocate memory you need both a size and an alignment.
    // Properly aligning our memory is necessary for a variety of reasons but suffice it to say that Rust takes care of this for us.
    let align = mem::align_of::<usize>();
    // The next thing we do is generate a memory layout for the particular size and alignment.
    // This can fail and return an error if the alignment is bad (zero or not a power of two)
    // or if size is too big, otherwise this should succeed.
    if let Ok(layout) = Layout::from_size_align(size, align) {
        unsafe {
            // Given our layout, we can then proceed to actually allocating memory.
            // If the resulting size of the layout is not positive
            // then we don’t need to allocate anything (and if fact calling alloc with a zero sized Layout could lead to undefined behavior depending on the architecture).
            // In this case we just cast the alignment to the right type and return it as that is about all we can do.
            if layout.size() > 0 {
                // Otherwise we have a real region of memory we need to allocate so we use the alloc function provided by the standard library.
                // We get back a pointer from alloc which is the location of our newly allocated region of memory
                // of the size and alignment specified by our layout.
                let ptr = alloc(layout);
                // We only return this pointer if it is not null. A null pointer returned from alloc most likely means you are out of memory.
                if !ptr.is_null() {
                    return ptr
                }
            } else {
                return align as *mut u8
            }
        }
    }

    panic!("malloc failed")
}

// Note that our entire function is marked as unsafe as part of the signature rather
// than putting an unsafe block inside the function.
// If you mark a function as unsafe, you are still responsible for making your invariants clear but
// you are saying that it is the responsibility of the caller to handle the bad cases.
#[no_mangle]
pub unsafe extern "C" fn __free(ptr: *mut u8, size: usize) {
    if size == 0 {
        return
    }
    // We take a pointer to the memory region we want to deallocate and the size of that region.
    // If the size is zero then there is nothing to do. Otherwise we do the reverse of allocation,
    // - we get an alignment,
    // - use that to get a Layout,
    // - and then pass the pointer and the layout to dealloc.
    let align = mem::align_of::<usize>();
    let layout = Layout::from_size_align_unchecked(size, align);
    dealloc(ptr, layout);
}

// We mark this function as unsafe for the same reason as __free.
// Box::from_raw is an unsafe function which takes a raw pointer and constructs a box from it.
#[no_mangle]
pub unsafe extern "C" fn __boxed_str_free(ptr: *mut String) {
    // By creating this box and putting it into a local variable we ensure that the Box will be dropped
    // at the end of the function body.
    // When the Box is dropped, as it is the sole owner of the String, the String will also be dropped.
    // The input type being *mut String is sufficient to tell Rust the right code to execute to drop both the Box
    // and the String as this drives the type inference of from_raw.
    let _b = Box::from_raw(ptr);
}