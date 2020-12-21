use criterion::{criterion_group, criterion_main, Criterion};
use wasmer::{imports, Function, Global, Instance, Module, Store, Value};

fn wasm_stack(c: &mut Criterion) {
    let store = Store::default();
    let module = Module::new(&store, WASM_STACK).unwrap();
    let sample_rate = 48_000.0;
    let import_object = imports! {
        "math" => {
            "sin" => Function::new_native(&store, sin),
            "modulo" => Function::new_native(&store, modulo),
            "pi" => Global::new(&store, Value::F64(std::f64::consts::PI)),
            "tau" => Global::new(&store, Value::F64(std::f64::consts::TAU)),
        },
        "audio" => {
            "sample_rate" => Global::new(&store, Value::F64(sample_rate)),
            "sample_period" => Global::new(&store, Value::F64(sample_rate.recip())),
        }
    };
    let dsp = Instance::new(&module, &import_object)
        .unwrap()
        .exports
        .get_native_function::<(), (f64, f64)>("dsp")
        .unwrap();
    c.bench_function("wasm stack", |b| b.iter(|| dsp.call().unwrap()));
}

fn memory_stack(c: &mut Criterion) {
    let store = Store::default();
    let module = Module::new(&store, MEMORY_STACK).unwrap();
    let sample_rate = 48_000.0;
    let import_object = imports! {
        "math" => {
            "sin" => Function::new_native(&store, sin),
            "modulo" => Function::new_native(&store, modulo),
            "pi" => Global::new(&store, Value::F64(std::f64::consts::PI)),
            "tau" => Global::new(&store, Value::F64(std::f64::consts::TAU)),
        },
        "audio" => {
            "sample_rate" => Global::new(&store, Value::F64(sample_rate)),
            "sample_period" => Global::new(&store, Value::F64(sample_rate.recip())),
        }
    };
    let dsp = Instance::new(&module, &import_object)
        .unwrap()
        .exports
        .get_native_function::<(), (f64, f64)>("dsp")
        .unwrap();
    c.bench_function("memory stack", |b| b.iter(|| dsp.call().unwrap()));
}

criterion_group!(benches, wasm_stack, memory_stack);
criterion_main!(benches);

pub fn sin(x: f64) -> f64 {
    x.sin()
}

pub fn modulo(x: f64, y: f64) -> f64 {
    x % y
}

const WASM_STACK: &str = r#"
(module
 (global $_sample_rate (import "audio" "sample_rate") f64)
 (global $_sample_period (import "audio" "sample_period") f64)
 (global $_pi (import "math" "pi") f64)
 (global $_tau (import "math" "tau") f64)
 (func $_sin (import "math" "sin") (param f64) (result f64))
 (func $_modulo (import "math" "modulo") (param f64 f64) (result f64))

 (memory 1)
 (table 256 funcref)

 (func $pi (result f64 f64)
       global.get $_pi
       global.get $_pi)

 (func $tau (result f64 f64)
       global.get $_tau
       global.get $_tau)

 (func $sample_rate (result f64 f64)
       global.get $_sample_rate
       global.get $_sample_rate)

 (func $sample_period (result f64 f64)
       global.get $_sample_period
       global.get $_sample_period)

 (func $sin (param f64 f64) (result f64 f64)
       local.get 0
       call $_sin
       local.get 1
       call $_sin)

 (func $add (param f64 f64 f64 f64) (result f64 f64)
       local.get 0
       local.get 2
       f64.add
       local.get 1
       local.get 3
       f64.add)

 (func $sub (param f64 f64 f64 f64) (result f64 f64)
       local.get 0
       local.get 2
       f64.sub
       local.get 1
       local.get 3
       f64.sub)

 (func $mul (param f64 f64 f64 f64) (result f64 f64)
       local.get 0
       local.get 2
       f64.mul
       local.get 1
       local.get 3
       f64.mul)

 (func $div (param f64 f64 f64 f64) (result f64 f64)
       local.get 0
       local.get 2
       f64.div
       local.get 1
       local.get 3
       f64.div)

 (func $modulo (param f64 f64 f64 f64) (result f64 f64)
       local.get 0
       local.get 2
       call $_modulo
       local.get 1
       local.get 3
       call $_modulo)

 (func $w_10
       (param f64 f64 f64 f64 )
       (result f64 f64 )
       local.get 0
       local.get 1
       local.get 2
       local.get 3

       (call $frame.dup)
       (call $frame.rot)
       (call $sample_period)
       (call $mul)
       (call $frame.swap)
       (call $frame.gload)
       (call $add)
       (f64.const 1) (f64.const 1)
       (call $add)
       (f64.const 2) (f64.const 2)
       (call $modulo)
       (f64.const 1) (f64.const 1)
       (call $sub)
       (call $frame.swap)
       (call $frame.gstore))

 (func $s_11
       (param f64 f64 f64 f64 )
       (result f64 f64 )
       local.get 0
       local.get 1
       local.get 2
       local.get 3

       (call $w_10)
       (call $tau)
       (call $mul)
       (call $sin))

 (func (export "dsp") (result f64 f64)
    (f64.const 1) (f64.const 1)
    (f64.const 0) (f64.const 0)
    (call $w_10)
    (f64.const 110) (f64.const 110)
    (call $mul)
    (f64.const 1) (f64.const 1)
    (call $s_11)
    (f64.const 1) (f64.const 1)
    (f64.const 2) (f64.const 2)
    (call $w_10)
    (f64.const 220) (f64.const 220)
    (call $mul)
    (f64.const 3) (f64.const 3)
    (call $s_11)
    (call $add)
    (f64.const 1) (f64.const 1)
    (f64.const 4) (f64.const 4)
    (call $w_10)
    (f64.const 440) (f64.const 440)
    (call $mul)
    (f64.const 5) (f64.const 5)
    (call $s_11)
    (call $add)
    (f64.const 1) (f64.const 1)
    (f64.const 6) (f64.const 6)
    (call $w_10)
    (f64.const 880) (f64.const 880)
    (call $mul)
    (f64.const 7) (f64.const 7)
    (call $s_11)
    (call $add)
    (f64.const 1) (f64.const 1)
    (f64.const 8) (f64.const 8)
    (call $w_10)
    (f64.const 55) (f64.const 55)
    (call $mul)
    (f64.const 9) (f64.const 9)
    (call $s_11)
    (call $add)
    (f64.const 0.1) (f64.const 0.1)
    (call $mul))

 (func $frame.drop (param f64 f64))

 (func $frame.dup (param f64 f64) (result f64 f64 f64 f64)
       local.get 0
       local.get 1
       local.get 0
       local.get 1)

 (func $frame.swap (param f64 f64 f64 f64) (result f64 f64 f64 f64)
       local.get 2
       local.get 3
       local.get 0
       local.get 1)

 (func $frame.rot (param f64 f64 f64 f64 f64 f64) (result f64 f64 f64 f64 f64 f64)
       local.get 2
       local.get 3
       local.get 4
       local.get 5
       local.get 0
       local.get 1)

 (func $frame.gload (param f64 f64) (result f64 f64)
       local.get 0
       i32.trunc_f64_s
       call $global.offset
       f64.load
       local.get 1
       i32.trunc_f64_s
       call $global.offset
       i32.const 8
       i32.add
       f64.load)

 (func $frame.gstore (param f64 f64 f64 f64) (result f64 f64)
       local.get 2
       i32.trunc_f64_s
       call $global.offset
       local.get 0
       f64.store
       local.get 3
       i32.trunc_f64_s
       call $global.offset
       i32.const 8
       i32.add
       local.get 1
       f64.store
       local.get 0
       local.get 1)

 (func $global.offset (param $index i32) (result i32)
       (i32.mul (i32.const 0x10) (local.get $index)))

 (elem (i32.const 0)
       func
       ))
"#;

const MEMORY_STACK: &str = r#"
(module
 (global $_sample_rate (import "audio" "sample_rate") f64)
 (global $_sample_period (import "audio" "sample_period") f64)
 (global $_pi (import "math" "pi") f64)
 (global $_tau (import "math" "tau") f64)
 (func $_sin (import "math" "sin") (param f64) (result f64))
 (func $_modulo (import "math" "modulo") (param f64 f64) (result f64))

 (global $tos (mut i32) (i32.const 0x0000))

 (memory 1)
 (table 256 funcref)

 (func $pi
       (call $frame.push (global.get $_pi) (global.get $_pi)))

 (func $tau
       (call $frame.push (global.get $_tau) (global.get $_tau)))

 (func $sample_rate
       (call $frame.push (global.get $_sample_rate) (global.get $_sample_rate)))

 (func $sample_period
       (call $frame.push (global.get $_sample_period) (global.get $_sample_period)))

 (func $sin (local $right f64)
       call $frame.pop
       local.set $right
       call $_sin
       local.get $right
       call $_sin
       call $frame.push)

 (func $add (local $a_right f64) (local $b_left f64) (local $b_right f64)
       call $frame.pop
       local.set $b_right
       local.set $b_left
       call $frame.pop
       local.set $a_right

       local.get $b_left
       f64.add
       local.get $a_right
       local.get $b_right
       f64.add
       call $frame.push)

 (func $sub (local $a_right f64) (local $b_left f64) (local $b_right f64)
       call $frame.pop
       local.set $b_right
       local.set $b_left
       call $frame.pop
       local.set $a_right

       local.get $b_left
       f64.sub
       local.get $a_right
       local.get $b_right
       f64.sub
       call $frame.push)

 (func $mul (local $a_right f64) (local $b_left f64) (local $b_right f64)
       call $frame.pop
       local.set $b_right
       local.set $b_left
       call $frame.pop
       local.set $a_right

       local.get $b_left
       f64.mul
       local.get $a_right
       local.get $b_right
       f64.mul
       call $frame.push)

 (func $div (local $a_right f64) (local $b_left f64) (local $b_right f64)
       call $frame.pop
       local.set $b_right
       local.set $b_left
       call $frame.pop
       local.set $a_right

       local.get $b_left
       f64.div
       local.get $a_right
       local.get $b_right
       f64.div
       call $frame.push)

 (func $modulo (local $a_right f64) (local $b_left f64) (local $b_right f64)
       call $frame.pop
       local.set $b_right
       local.set $b_left
       call $frame.pop
       local.set $a_right

       local.get $b_left
       call $_modulo
       local.get $a_right
       local.get $b_right
       call $_modulo
       call $frame.push)

 (func $w_10
       (call $frame.dup)
       (call $frame.rot)
       (call $sample_period)
       (call $mul)
       (call $frame.swap)
       (call $frame.gload)
       (call $add)
       (call $frame.push (f64.const 1) (f64.const 1))
       (call $add)
       (call $frame.push (f64.const 2) (f64.const 2))
       (call $modulo)
       (call $frame.push (f64.const 1) (f64.const 1))
       (call $sub)
       (call $frame.swap)
       (call $frame.gstore))

 (func $s_11
       (call $w_10)
       (call $tau)
       (call $mul)
       (call $sin))

 (func (export "dsp") (result f64 f64)
    (global.set $tos (i32.const 0x0000))

    (call $frame.push (f64.const 1) (f64.const 1))
    (call $frame.push (f64.const 0) (f64.const 0))
    (call $w_10)
    (call $frame.push (f64.const 110) (f64.const 110))
    (call $mul)
    (call $frame.push (f64.const 1) (f64.const 1))
    (call $s_11)
    (call $frame.push (f64.const 1) (f64.const 1))
    (call $frame.push (f64.const 2) (f64.const 2))
    (call $w_10)
    (call $frame.push (f64.const 220) (f64.const 220))
    (call $mul)
    (call $frame.push (f64.const 3) (f64.const 3))
    (call $s_11)
    (call $add)
    (call $frame.push (f64.const 1) (f64.const 1))
    (call $frame.push (f64.const 4) (f64.const 4))
    (call $w_10)
    (call $frame.push (f64.const 440) (f64.const 440))
    (call $mul)
    (call $frame.push (f64.const 5) (f64.const 5))
    (call $s_11)
    (call $add)
    (call $frame.push (f64.const 1) (f64.const 1))
    (call $frame.push (f64.const 6) (f64.const 6))
    (call $w_10)
    (call $frame.push (f64.const 880) (f64.const 880))
    (call $mul)
    (call $frame.push (f64.const 7) (f64.const 7))
    (call $s_11)
    (call $add)
    (call $frame.push (f64.const 1) (f64.const 1))
    (call $frame.push (f64.const 8) (f64.const 8))
    (call $w_10)
    (call $frame.push (f64.const 55) (f64.const 55))
    (call $mul)
    (call $frame.push (f64.const 9) (f64.const 9))
    (call $s_11)
    (call $add)
    (call $frame.push (f64.const 0.1) (f64.const 0.1))
    (call $mul)

    call $frame.peek)

 (func $frame.drop
       (global.set $tos (i32.sub (global.get $tos) (i32.const 0x10))))

 (func $frame.dup
       call $frame.peek
       call $frame.push)

 (func $frame.swap (local $left f64) (local $right f64)
       call $frame.pop
       local.set $right
       local.set $left
       call $frame.pop
       local.get $left
       local.get $right
       call $frame.push
       call $frame.push)

 (func $frame.rot
       (local $a_left f64) (local $a_right f64)
       (local $b_left f64) (local $b_right f64)
       call $frame.pop
       local.set $a_right
       local.set $a_left
       call $frame.pop
       local.set $b_right
       local.set $b_left
       call $frame.pop
       local.get $b_left
       local.get $b_right
       call $frame.push
       local.get $a_left
       local.get $a_right
       call $frame.push
       call $frame.push)

 (func $frame.gload (local $right i32)
       call $frame.pop
       i32.trunc_f64_s
       call $global.offset
       local.set $right
       i32.trunc_f64_s
       call $global.offset
       f64.load
       local.get $right
       f64.load
       call $frame.push)

 (func $frame.gstore
       (local $left i32) (local $right i32)
       (local $x_left f64) (local $x_right f64)
       call $frame.pop
       i32.trunc_f64_s
       call $global.offset
       local.set $right
       i32.trunc_f64_s
       call $global.offset
       local.set $left

       call $frame.peek
       local.set $x_right
       local.set $x_left

       local.get $left
       local.get $x_left
       f64.store
       local.get $right
       local.get $x_right
       f64.store)

 (func $frame.pop (result f64 f64)
       call $frame.peek
       (global.set $tos (i32.sub (global.get $tos) (i32.const 0x10))))

 (func $frame.push (param $left f64) (param $right f64)
       global.get $tos
       local.get $left
       f64.store
       (i32.add (global.get $tos) (i32.const 0x08))
       local.get $right
       f64.store
       (global.set $tos (i32.add (global.get $tos) (i32.const 0x10))))

 (func $frame.peek (result f64 f64)
       (i32.sub (global.get $tos) (i32.const 0x10))
       f64.load
       (i32.sub (global.get $tos) (i32.const 0x08))
       f64.load)

 (func $i32.push (param $value i32)
       global.get $tos
       local.get $value
       i32.store
       (global.set $tos (i32.add (global.get $tos) (i32.const 0x10))))

 (func $global.offset (param $index i32) (result i32)
       (i32.add
        (i32.mul (i32.const 0x10) (local.get $index))
        (i32.const 0x1000)))

 (elem (i32.const 0)
       func
       ))
"#;
