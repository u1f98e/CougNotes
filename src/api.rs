use wasmedge_quickjs::*;
use crate::{student::*, class::*, post::*};

struct StudentDef;

impl JsClassDef<Student> for StudentDef {
	const CLASS_NAME: &'static str = "Student\0";
	const CONSTRUCTOR_ARGC: u8 = 2;

	fn constructor(_: &mut Context, argv: &[JsValue]) -> Option<Student> {
		println!("rust-> new Student {:?}", argv);
		let x = argv.get(0);
		let y = argv.get(1);
		if let ((Some(JsValue::Int(ref x)), Some(JsValue::Int(ref y)))) = (x, y) {
			Some(Student())
		} else {
			None
		}
	}

	fn proto_init(p: &mut JsClassProto<Student, StudentDef>) {
		struct X;
		impl JsClassGetterSetter<Student> for X {
			const NAME: &'static str = "x\0";

			fn getter(_: &mut Context, this_val: &mut Student) -> JsValue {
				println!("rust-> get x");
				this_val.0.into()
			}

			fn setter(_: &mut Context, this_val: &mut Student, val: JsValue) {
				println!("rust-> set x:{:?}", val);
				if let JsValue::Int(x) = val {
					this_val.0 = x
				}
			}
		}

		struct Y;
		impl JsClassGetterSetter<Student> for Y {
			const NAME: &'static str = "y\0";

			fn getter(_: &mut Context, this_val: &mut Student) -> JsValue {
				println!("rust-> get y");
				this_val.1.into()
			}

			fn setter(_: &mut Context, this_val: &mut Student, val: JsValue) {
				println!("rust-> set y:{:?}", val);
				if let JsValue::Int(y) = val {
					this_val.1 = y
				}
			}
		}

		struct FnPrint;
		impl JsMethod<Student> for FnPrint {
			const NAME: &'static str = "pprint\0";
			const LEN: u8 = 0;

			fn call(_: &mut Context, this_val: &mut Student, _argv: &[JsValue]) -> JsValue {
				println!("rust-> pprint: {:?}", this_val);
				JsValue::Int(1)
			}
		}

		p.add_getter_setter(X);
		p.add_getter_setter(Y);
		p.add_function(FnPrint);
	}

}

struct MainModule;
impl ModuleInit for MainModule {
	fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
		m.add_export("Student\0", StudentDef::class_value(ctx));
	}
}

pub fn init_main_module(ctx: &mut Context) {
	ctx.register_class(StudentDef);
	ctx.register_module("student\0", MainModule, &["Student\0"]);

	// ctx.register_class(ClassDef);
	// ctx.register_module("class\0", NotesModule, &["Class\0"]);
    //
	// ctx.register_class(PostDef);
	// ctx.register_module("post\0", NotesModule, &["Post\0"]);
}