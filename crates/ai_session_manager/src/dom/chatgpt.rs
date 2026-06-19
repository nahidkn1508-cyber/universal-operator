pub const CHATGPT_LAST_RESPONSE: &str = r#"
(() => {

const messages =
document.querySelectorAll('[data-message-author-role="assistant"]');

if(messages.length===0)
    return "";

return messages[messages.length-1].innerText;

})();
"#;
