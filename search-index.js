var searchIndex = JSON.parse('{\
"as_ffi_bindings":{"doc":"","t":[3,3,4,13,8,3,8,5,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,12,12,12,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,12,11,11,11,11,11,11,11,11,11,10,11,11,10,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,11,10,11,11,12],"n":["BufferPtr","Env","Error","Mem","Read","StringPtr","Write","abort","alloc","alloc","alloc","borrow","borrow","borrow","borrow","borrow_mut","borrow_mut","borrow_mut","borrow_mut","clone","clone","clone","clone_into","clone_into","clone_into","default","deref","deref","deref","deref","deref_mut","deref_mut","deref_mut","deref_mut","deserialize","deserialize","deserialize","deserialize","drop","drop","drop","drop","empty_array","empty_array","fmt","fmt","fn_collect","fn_new","fn_pin","fn_unpin","free","free","free","from","from","from","from","from_array","from_array","from_c_struct","from_c_struct","from_native","from_native","from_slice","from_slice","init","init","init","init","init","init_with_instance","into","into","into","into","into_array","into_array","into_c_struct","into_c_struct","memory","new","new","new","offset","offset","pointer_metadata","pointer_metadata","pointer_metadata","pointer_metadata","read","read","read","size","size","size","to_native","to_native","to_owned","to_owned","to_owned","to_string","try_from","try_from","try_from","try_from","try_into","try_into","try_into","try_into","type_id","type_id","type_id","type_id","wasm_types","wasm_types","write","write","write","0"],"q":["as_ffi_bindings","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","as_ffi_bindings::Error"],"d":["","","","","","","","","Try to write in the given environment a new value thanks …","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","","Unpin the pointer","","","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","Returns the argument unchanged.","","","","","","","","","","","","","","","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","Calls <code>U::from(self)</code>.","","","","","","","","","","","","","","","Read the value contained in the given memory at the …","","","Read the size as indicated in the AssemblyScript object …","","","","","","","","","","","","","","","","","","","","","","","Try to write in the given environment a value. If the size …","","",""],"i":[0,0,0,1,0,0,0,0,2,3,4,3,5,4,1,3,5,4,1,3,5,4,3,5,4,5,3,5,4,1,3,5,4,1,3,5,4,1,3,5,4,1,3,4,1,1,5,5,5,5,2,3,4,3,5,4,1,3,4,3,4,3,4,3,4,3,5,5,4,1,5,3,5,4,1,3,4,3,4,5,3,5,4,3,4,3,5,4,1,6,3,4,6,3,4,3,4,3,5,4,1,3,5,4,1,3,5,4,1,3,5,4,1,3,4,2,3,4,7],"f":[null,null,null,null,null,null,null,[[["env",3],["stringptr",3],["stringptr",3],["i32",0],["i32",0]],["result",4,[["runtimeerror",3]]]],[[["",0],["env",3]],["result",6,[["box",3]]]],[[["vec",3],["env",3]],["result",6,[["box",3,[["bufferptr",3]]]]]],[[["string",3],["env",3]],["result",6,[["box",3,[["stringptr",3]]]]]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["",0]],[[["",0]],["bufferptr",3]],[[["",0]],["env",3]],[[["",0]],["stringptr",3]],[[["",0],["",0]]],[[["",0],["",0]]],[[["",0],["",0]]],[[],["env",3]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["usize",0]],["",0]],[[["",0],["",0]],["result",4,[["with",3]]]],[[["",0],["",0]],["result",4,[["with",3]]]],[[["",0],["",0]],["result",4,[["with",3]]]],[[["",0],["",0]],["result",4,[["with",3]]]],[[["usize",0]]],[[["usize",0]]],[[["usize",0]]],[[["usize",0]]],[[]],[[]],[[["",0],["formatter",3]],["result",6]],[[["",0],["formatter",3]],["result",6]],null,null,null,null,[[["env",3]],["result",6]],[[["env",3]],["result",6]],[[["env",3]],["result",6]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[],["result",4,[["tryfromsliceerror",3]]]],[[],["result",4,[["tryfromsliceerror",3]]]],[[],["usize",0]],[[],["usize",0]],[[["",0],["instance",3]],["result",6]],[[],["usize",0]],[[],["usize",0]],[[["",0],["instance",3]],["result",4,[["hostenviniterror",4]]]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],[[]],null,[[["u32",0]]],[[["memory",3],["option",4,[["function",3]]],["option",4,[["function",3]]],["option",4,[["function",3]]],["option",4,[["function",3]]]],["env",3]],[[["u32",0]]],[[["",0]],["u32",0]],[[["",0]],["u32",0]],[[]],[[]],[[]],[[]],[[["",0],["memory",3]],["result",6]],[[["",0],["memory",3]],["result",6,[["vec",3,[["u8",0]]]]]],[[["",0],["memory",3]],["result",6,[["string",3]]]],[[["",0],["memory",3]],["result",6,[["u32",0]]]],[[["",0],["memory",3]],["result",6,[["u32",0]]]],[[["",0],["memory",3]],["result",6,[["u32",0]]]],[[]],[[]],[[["",0]]],[[["",0]]],[[["",0]]],[[["",0]],["string",3]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[],["result",4]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[["",0]],["typeid",3]],[[]],[[]],[[["",0],["",0],["env",3]],["result",6,[["box",3]]]],[[["",0],["vec",3],["env",3]],["result",6,[["box",3]]]],[[["",0],["string",3],["env",3]],["result",6,[["box",3,[["stringptr",3]]]]]],null],"p":[[4,"Error"],[8,"Write"],[3,"BufferPtr"],[3,"StringPtr"],[3,"Env"],[8,"Read"],[13,"Mem"]]}\
}');
if (window.initSearch) {window.initSearch(searchIndex)};