mod api;
use wasmedge_quickjs::*;

fn main() {
    let mut ctx = Context::new();
    api::init_main_module(&mut ctx);

    let code = r#"
    import('point').then((point)=>{
        let p0 = new point.Point(1,2)
        print("js->",p0.x,p0.y)
        p0.pprint()

        try{
            let p = new point.Point()
            print("js-> p:",p)
            print("js->",p.x,p.y)
            p.x=2
            p.pprint()
        } catch(e) {
            print("An error has been caught");
            print(e)
        }
        
    })
    "#;

    ctx.eval_global_str(code);
    ctx.promise_loop_poll();
}

