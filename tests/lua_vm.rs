use lua_ffi::{LuaConfig, LuaError, LuaStdLib, LuaValue, LuaVm};

mod construction {
    use super::*;

    #[test]
    fn new_vm_runs_arithmetic() {
        let vm = LuaVm::new();
        assert!(matches!(vm.eval("return 1 + 1".into()).unwrap(), LuaValue::Integer(2)));
    }

    #[test]
    fn version_is_not_empty() {
        let vm = LuaVm::new();
        assert!(!vm.version().is_empty());
    }

    #[test]
    fn with_config_all_stdlib() {
        let vm = LuaVm::with_config(LuaConfig { stdlib: LuaStdLib::All }).unwrap();
        let v = vm.eval("return type(io)".into()).unwrap();
        assert!(matches!(v, LuaValue::LuaString(s) if s == "table"));
    }

    #[test]
    fn with_config_safe_stdlib_no_io() {
        let vm = LuaVm::with_config(LuaConfig { stdlib: LuaStdLib::Safe }).unwrap();
        let v = vm.eval("return io".into()).unwrap();
        assert!(matches!(v, LuaValue::Nil));
    }

    #[test]
    fn with_config_none_stdlib() {
        let vm = LuaVm::with_config(LuaConfig { stdlib: LuaStdLib::None }).unwrap();
        let v = vm.eval("return tostring".into()).unwrap();
        assert!(matches!(v, LuaValue::Nil));
    }
}

mod exec {
    use super::*;

    #[test]
    fn sets_global() {
        let vm = LuaVm::new();
        vm.exec("answer = 42".into()).unwrap();
        assert!(matches!(vm.get_global("answer".into()).unwrap(), LuaValue::Integer(42)));
    }

    #[test]
    fn returns_unit() {
        let vm = LuaVm::new();
        assert!(vm.exec("local x = 1 + 1".into()).is_ok());
    }
}

mod eval {
    use super::*;

    #[test]
    fn nil() {
        let vm = LuaVm::new();
        assert!(matches!(vm.eval("return nil".into()).unwrap(), LuaValue::Nil));
    }

    #[test]
    fn boolean_true() {
        let vm = LuaVm::new();
        assert!(matches!(vm.eval("return true".into()).unwrap(), LuaValue::Boolean(true)));
    }

    #[test]
    fn boolean_false() {
        let vm = LuaVm::new();
        assert!(matches!(vm.eval("return false".into()).unwrap(), LuaValue::Boolean(false)));
    }

    #[test]
    fn integer() {
        let vm = LuaVm::new();
        assert!(matches!(vm.eval("return 99".into()).unwrap(), LuaValue::Integer(99)));
    }

    #[test]
    fn negative_integer() {
        let vm = LuaVm::new();
        assert!(matches!(vm.eval("return -7".into()).unwrap(), LuaValue::Integer(-7)));
    }

    #[test]
    fn float() {
        let vm = LuaVm::new();
        let LuaValue::Number(n) = vm.eval("return 3.14".into()).unwrap() else {
            panic!("expected Number");
        };
        assert!((n - 3.14).abs() < f64::EPSILON);
    }

    #[test]
    fn string() {
        let vm = LuaVm::new();
        assert!(matches!(
            vm.eval(r#"return "hello""#.into()).unwrap(),
            LuaValue::LuaString(s) if s == "hello"
        ));
    }

    #[test]
    fn no_return_is_nil() {
        let vm = LuaVm::new();
        assert!(matches!(vm.eval("local x = 1".into()).unwrap(), LuaValue::Nil));
    }
}

mod globals {
    use super::*;

    #[test]
    fn set_get_nil() {
        let vm = LuaVm::new();
        vm.set_global("x".into(), LuaValue::Nil).unwrap();
        assert!(matches!(vm.get_global("x".into()).unwrap(), LuaValue::Nil));
    }

    #[test]
    fn set_get_boolean() {
        let vm = LuaVm::new();
        vm.set_global("flag".into(), LuaValue::Boolean(true)).unwrap();
        assert!(matches!(vm.get_global("flag".into()).unwrap(), LuaValue::Boolean(true)));
    }

    #[test]
    fn set_get_integer() {
        let vm = LuaVm::new();
        vm.set_global("n".into(), LuaValue::Integer(123)).unwrap();
        assert!(matches!(vm.get_global("n".into()).unwrap(), LuaValue::Integer(123)));
    }

    #[test]
    fn set_get_float() {
        let vm = LuaVm::new();
        vm.set_global("f".into(), LuaValue::Number(2.718)).unwrap();
        let LuaValue::Number(n) = vm.get_global("f".into()).unwrap() else {
            panic!("expected Number");
        };
        assert!((n - 2.718).abs() < f64::EPSILON);
    }

    #[test]
    fn set_get_string() {
        let vm = LuaVm::new();
        vm.set_global("s".into(), LuaValue::LuaString("world".into())).unwrap();
        assert!(matches!(
            vm.get_global("s".into()).unwrap(),
            LuaValue::LuaString(s) if s == "world"
        ));
    }

    #[test]
    fn visible_in_script() {
        let vm = LuaVm::new();
        vm.set_global("base".into(), LuaValue::Integer(10)).unwrap();
        assert!(matches!(
            vm.eval("return base * 2".into()).unwrap(),
            LuaValue::Integer(20)
        ));
    }

    #[test]
    fn missing_is_nil() {
        let vm = LuaVm::new();
        assert!(matches!(vm.get_global("does_not_exist".into()).unwrap(), LuaValue::Nil));
    }
}

mod errors {
    use super::*;

    #[test]
    fn syntax_error_variant() {
        let vm = LuaVm::new();
        let err = vm.exec("this is not valid lua !!!".into()).unwrap_err();
        assert!(matches!(err, LuaError::Syntax { .. }));
    }

    #[test]
    fn runtime_error_variant() {
        let vm = LuaVm::new();
        let err = vm.exec("error('boom')".into()).unwrap_err();
        assert!(matches!(err, LuaError::Runtime { .. }));
    }

    #[test]
    fn runtime_error_message_preserved() {
        let vm = LuaVm::new();
        let err = vm.exec("error('something went wrong')".into()).unwrap_err();
        assert!(err.to_string().contains("something went wrong"));
    }

    #[test]
    fn syntax_error_message_preserved() {
        let vm = LuaVm::new();
        let err = vm.eval("???".into()).unwrap_err();
        let LuaError::Syntax { msg } = err else {
            panic!("expected Syntax, got {err}");
        };
        assert!(!msg.is_empty());
    }
}