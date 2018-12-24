command!(convert(_ctx, msg, args) {
    let value = args.single::<f64>().unwrap();
    let curr_one = args.single::<String>().unwrap();
    let curr_two = args.single::<String>().unwrap();


});