use typed_html::{html,dom::*,text};

pub fn create_mail(link:String)->String{
    let doc: DOMTree<String> = html!(
        <html>
            <head>
                <title>"Account Verification"</title>
            </head>
            <body>
                <h3 style="color:grey">{text!("Click here to verify your account : {}",link)}</h3>
            </body>
        </html>
    );

    doc.to_string()
}