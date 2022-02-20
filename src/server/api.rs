// use wasmedge_quickjs::*;
// use sqlx::mysql::MySqlPool;
// use crate::{student::*, class::*, post::*};
// use sqlx::mysql::MySqlPoolOptions as PoolOptions;
//
// struct StudentDef;
//
// impl JsClassDef<Student> for StudentDef {
// 	const CLASS_NAME: &'static str = "Student\0";
// 	const CONSTRUCTOR_ARGC: u8 = 4;
//
// 	fn constructor(_: &mut Context, argv: &[JsValue]) -> Option<Student> {
// 		let student_id = argv.get(0);
// 		let first      = argv.get(1);
// 		let last       = argv.get(2);
// 		let img_url    = argv.get(3);
// 		if let (
// 				Some(JsValue::String(student_id)),
// 				Some(JsValue::String(first)),
// 				Some(JsValue::String(last)),
// 				Some(JsValue::String(img_url)),
// 			) = (student_id, first, last, img_url) 
// 		{
// 			Some(Student::new(
// 					student_id.to_string(),
// 					first.to_string(),
// 					last.to_string(),
// 					img_url.to_string()
// 				)
// 			)
// 		} else {
// 			None
// 		}
// 	}
//
// 	fn proto_init(p: &mut JsClassProto<Student, StudentDef>) {
// 		let test = 123;
// 		struct X;
// 		impl JsClassGetterSetter<Student> for X {
// 			const NAME: &'static str = "x\0";
//
// 			fn getter(_: &mut Context, this_val: &mut Student) -> JsValue {
// 				this_val.0.into()
// 			}
//
// 			fn setter(_: &mut Context, this_val: &mut Student, val: JsValue) {
// 				if let JsValue::Int(x) = val {
// 					this_val.0 = x
// 				}
// 			}
// 		}
//
// 		struct Y;
// 		impl JsClassGetterSetter<Student> for Y {
// 			const NAME: &'static str = "y\0";
//
// 			fn getter(_: &mut Context, this_val: &mut Student) -> JsValue {
// 				println!("rust-> get y");
// 				this_val.1.into()
// 			}
//
// 			fn setter(_: &mut Context, this_val: &mut Student, val: JsValue) {
// 				println!("rust-> set y:{:?}", val);
// 				if let JsValue::Int(y) = val {
// 					this_val.1 = y
// 				}
// 			}
// 		}
//
// 		struct FnPrint;
// 		impl JsMethod<Student> for FnPrint {
// 			const NAME: &'static str = "pprint\0";
// 			const LEN: u8 = 0;
//
// 			fn call(_: &mut Context, this_val: &mut Student, _argv: &[JsValue]) -> JsValue {
// 				|| {
// 					JsValue::int(test)
// 				}
// 			}
// 			// fn call(_: &mut Context, this_val: &mut Student, _argv: &[JsValue]) -> JsValue {
// 			// 	println!("rust-> pprint: {:?}", this_val);
// 			// 	println!("{}", test);
// 			// 	JsValue::Int(1)
// 			// }
// 		}
//
// 		struct FnCreateStudent;
// 		impl JsMethod<Student> for FnCreateStudent {
// 			const NAME: &'static str = "create_student\0";
// 			const LEN: u8 = 0;
//
// 			fn call(_: &mut Context, this_val: &mut Student, _argv: &[JsValue]) -> JsValue {
// 				this_val.
// 				JsValue::Int(1)
// 			}
// 		}
//
// 		p.add_getter_setter(X);
// 		p.add_getter_setter(Y);
// 		p.add_function(FnPrint);
// 	}
//
// }
//
// struct MainModule {
// 	sql_pool: PoolOptions
// }
//
// pub fn init_main_module(ctx: &mut Context, sql_pool: PoolOptions) {
// 	let module = MainModule {
// 		sql_pool
// 	};
//
// 	ctx.register_class(StudentDef);
// 	ctx.register_class(ClassDef);
// 	ctx.register_class(PostDef);
// 	ctx.register_module("student\0", module, &["Student\0", "Class\0", "Post\0"]);
// }
//
// impl ModuleInit for MainModule {
// 	fn init_module(ctx: &mut Context, m: &mut JsModuleDef) {
// 		m.add_export("Student\0", StudentDef::class_value(ctx));
// 		m.add_export("Class\0", ClassDef::class_value(ctx));
// 		m.add_export("Post\0", PostDef::class_value(ctx));
// 	}
// }
//