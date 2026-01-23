use dioxus::prelude::*;
use lucide_dioxus::ArrowLeft;

#[component]
pub fn Privacy() -> Element {
    rsx! {
        div { class: "flex items-center py-4 bg-background px-4 max-w-6xl w-full",
            a {
                class: "flex items-center justify-center bg-primary text-primary-foreground gap-2 font-bold py-2 px-4 w-fit rounded hover:bg-opacity-90 text-lg",
                href: "/",
                ArrowLeft { class: "w-5 h-5" }
                "Back to Home"
            }
        }
        div { class: "mt-6 w-full flex flex-col gap-6 px-4 max-w-6xl justify-center",
            section { class: "flex flex-col gap-4",
                h1 { class: "text-3xl font-bold", "Privacy Policy" }
                p { "Effective Date: January 12, 2026" }
                p {
                    "At Stop Communism PAC (SCP), we are committed to safeguarding the privacy of our customers and users. This Privacy Policy outlines our practices regarding the collection, use, and protection of your personal information. By using our website or providing your personal information to us, you consent to the practices described in this policy."
                }
            }
            section { class: "flex flex-col gap-4",
                h2 { class: "text-2xl font-bold", "1. Information We Collect" }
                p { "We collect the following types of information:" }
                h3 { class: "text-xl font-bold", "1.1 Personal Information" }
                p {
                    "When you use our website, submit inquiries, or register for our services, we may collect personal information, including but not limited to your name, email address, postal address, and telephone number."
                }
                h3 { class: "text-xl font-bold", "1.2 Cell Phone Data" }
                p {
                    "If you choose to provide your cell phone number for the purpose of direct communication, we may collect and store this information to better serve your needs."
                }
                h3 { class: "text-xl font-bold", "1.3 Opt-In Data" }
                p {
                    "If you opt in to receive communication, newsletters, or updates from SCP, we will collect and use your contact information to send you the requested information. You may unsubscribe or opt out of these communications at any time."
                }
            }
            section { class: "flex flex-col gap-4",
                h2 { class: "text-2xl font-bold", "2. How We Use Your Information" }
                p { "We use the information collected for the following purposes:" }
                h3 { class: "text-xl font-bold", "2.1 Direct Communication" }
                p {
                    "We use your information to communicate with you, respond to your inquiries, and provide you with updates about our products and services."
                }
                h3 { class: "text-xl font-bold", "2.2 Opt-In Communications" }
                p {
                    "If you have opted in to receive marketing communications, we use your contact information to send you relevant content, offers, and updates."
                }
                h3 { class: "text-xl font-bold", "2.3 Website Improvement" }
                p {
                    "We analyze data about the use of our website to improve its functionality and user experience."
                }
            }
            section { class: "flex flex-col gap-4",
                h2 { class: "text-2xl font-bold", "3. Data Protection and Sharing" }
                p { "At SCP, we are committed to protecting your data:" }
                h3 { class: "text-xl font-bold", "3.1 Data Security" }
                p {
                    "We implement appropriate security measures to protect your personal information from unauthorized access, disclosure, or misuse."
                }
                h3 { class: "text-xl font-bold", "3.2 No Sale or Sharing" }
                p {
                    "We will never sell your personal data or share it with third-party marketing partners. Your information will only be used for direct communication with SCP users. SMS opt-in consent and data will not be shared with third parties."
                }
                h3 { class: "text-xl font-bold", "3.3 Text Messaging Opt-In Data" }
                p {
                    "We will not share or sell your text messaging opt-in data, consent, or related personal information with any third parties, unless required by law."
                }
            }
            section { class: "flex flex-col gap-4",
                h2 { class: "text-2xl font-bold", "4. Your Choices" }
                h3 { class: "text-xl font-bold", "4.1 Opting Out" }
                p {
                    "You have the option to opt out of any marketing communications by clicking the “unsubscribe” link in our emails or by contacting us directly. To opt out of text messages, simply reply “STOP”."
                }
                h3 { class: "text-xl font-bold", "4.2 Updating Your Information" }
                p {
                    "If you need to update or correct your personal information, please contact us at info@stopcommunism.org."
                }
            }
            section { class: "flex flex-col gap-4",
                h2 { class: "text-2xl font-bold", "5. Changes to this Privacy Policy" }
                h3 { class: "text-xl font-bold", "5.1 Updates" }
                p {
                    "We may update this Privacy Policy from time to time to reflect changes in our practices and legal requirements. Any updates will be posted on our website, and the date at the beginning of the policy will be revised accordingly."
                }
            }
            section { class: "flex flex-col gap-4",
                h2 { class: "text-2xl font-bold", "6. Contact Us" }
                h3 { class: "text-xl font-bold", "6.1 Contact Information" }
                p {
                    "If you have any questions or concerns about this Privacy Policy or how we handle your personal information, please contact us at: info@stopcommunism.org."
                }
                p {
                    "By using our website or providing your information to SCP, you acknowledge and agree to the terms outlined in this Privacy Policy. Your privacy and data security are of the utmost importance to us, and we are dedicated to protecting your personal information in accordance with this policy."
                }
            }
            section { class: "flex flex-col gap-4",
                h2 { class: "text-2xl font-bold", "7. SMS Terms and Conditions" }
                p {
                    "By entering your phone number and selecting to opt in, you consent to join a recurring SMS/MMS text messaging program that will provide alerts, donation requests, updates, and other important information. By participating, you agree to the terms & privacy policy for auto dialed messages to the phone number you provide. Msg&data rates may apply. Msg frequency varies. Reply HELP for help or STOP to opt-out at any time. SMS information is not rented, sold, or shared."
                }
            }
        }
    }
}
