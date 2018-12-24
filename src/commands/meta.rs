command!(help(_ctx, msg) {
    if let Err(why) = msg.channel_id.send_message(|m| m
        .embed(|e| e
            .title("Hi there! I'm Xbot.")
            .description("Here's a helpful list of some common commands.")
            .fields(vec![
                ("General Commands", "~help\n", true),
                ("Description", "Brings up this dialog\n", true),
            ])
            .footer(|f| f
                .text("Created by Kaironaut. Ver. 0.0.1")))) {
        println!("Error sending message: {:?}", why);
    }
});