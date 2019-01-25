use reqwest::Client;

#[derive(Deserialize, Debug)]
struct Currency {
    from: String,
    to: String,
    result: f64,
}

command!(convert(_ctx, msg, args) {
    let value = args.single::<f64>().unwrap();
    let curr_one = args.single::<String>().unwrap();
    let curr_two = args.single::<String>().unwrap();

    let request_url = format!("http://data.fixer.io/api/convert
                              ?access_key={}
                              &from={}
                              &to={}
                              &amount={}",
                              "49c3695a05794ed92cadf05df6b4d9cc", curr_one, curr_two, value);

    let mut response = reqwest::get(&request_url)?;
    let ans: Currency = response.json()?;
    let answer = format!("{}", ans.result);

    let _ = msg.reply(&answer);
});