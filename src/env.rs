use wasmer::{Function, Instance, Memory};

#[derive(Clone, Default)]
pub struct Env0 {
    pub memory: Option<Memory>,
}

#[derive(Clone, Default)]
pub struct Env<'a> {
    pub memory: Option<Memory>,
    pub fn_new: Option<&'a Function>,
    pub fn_pin: Option<&'a Function>,
    pub fn_unpin: Option<&'a Function>,
    pub fn_collect: Option<&'a Function>,
}

impl<'a> Env<'a> {
    /*
    pub fn new(
        // arg_memory: Memory,
        fn_new: Option<&Function>,
        fn_pin: Option<&Function>,
        fn_unpin: Option<&Function>,
        fn_collect: Option<&Function>,
    ) -> Self {
        // let mut memory = LazyInit::<Memory>::default();
        // memory.initialize(arg_memory);
        Env {
            // memory,
            fn_new,
            fn_pin,
            fn_unpin,
            fn_collect,
        }
    }
    */

    pub fn init(&mut self, instance: &'a Instance) -> anyhow::Result<()> {
        // Ok(self.init_with_instance(instance)?)

        self.fn_new = Some(instance.exports.get_function("__new")?);
        self.fn_pin = Some(instance.exports.get_function("__pin")?);
        self.fn_unpin = Some(instance.exports.get_function("__unpin")?);
        self.fn_collect = Some(instance.exports.get_function("__collect")?);
        Ok(())
    }
}

/*
impl TryFrom<&Instance> for Env {
    type Error = anyhow::Error;

    fn try_from(instance: &Instance) -> Result<Self, Self::Error> {
        Ok(Self {
            // memory: instance.exports.get_with_generics("memory")?,
            // memory: Some(instance.exports.get_memory("memory")?),
            // fn_new: Some(instance.exports.get_function("__new")?),
            // fn_pin: Some(instance.exports.get_function("__pin")?),
            // fn_unpin: Some(instance.exports.get_function("__unpin")?),
            // fn_collect: Some(instance.exports.get_function("__collect")?),
        })
    }
}
*/

/*
impl WasmerEnv for Env {
    fn init_with_instance(&mut self, instance: &Instance) -> Result<(), HostEnvInitError> {
        let mem: Memory = instance
            .exports
            .get_with_generics_weak("memory")
            .map_err(HostEnvInitError::from)?;
        if let Ok(func) = instance.exports.get_with_generics_weak("__new") {
            self.fn_new = Some(func)
        }
        if let Ok(func) = instance.exports.get_with_generics_weak("__pin") {
            self.fn_pin = Some(func)
        }
        if let Ok(func) = instance.exports.get_with_generics_weak("__unpin") {
            self.fn_unpin = Some(func)
        }
        if let Ok(func) = instance.exports.get_with_generics_weak("__collect") {
            self.fn_collect = Some(func)
        }
        self.memory.initialize(mem);
        Ok(())
    }
}
*/
