
use yew::prelude::*;


#[derive(Clone, PartialEq)]
struct Link {
    name: String,
    url: String,
}

#[derive(Properties, PartialEq)]
struct LinkListProps {
    links: Vec<Link>,
}
#[function_component(LinkList)]
fn link_list(LinkListProps { links }: &LinkListProps) -> Html {
    links
        .iter()
        .map(|link| {
            html! {
                 <div class="link-div">
                    <div>
                        <a href={format!("{}", link.url)}>
                            {format!("{}",link.name)}
                        </a>
                    </div>
                </div>
            }
        })
        .collect()
}


#[function_component]
fn App() -> Html {

    let links = vec![
        Link {
            name: "Steam".to_string(),
            url: "https://steamcommunity.com/id/RichPotato".to_string(),
        },
        Link {
            name: "Telegram".to_string(),
            url: "https://t.me/richpotato".to_string(),
        },
        Link {
            name: "Github".to_string(),
            url: "https://github.com/rilr20".to_string(),
        },
        Link {
            name: "Letterboxd".to_string(),
            url: "https://letterboxd.com/Palmemordet/".to_string(),
        },
    ];

    let my_header: Html = html! {
        <div>
            <h1>{"Links 'n' Stuff"}</h1>
        </div>
    };

    html! {
        <div>
            {my_header}
            <div class="link-container">
                <LinkList links={links} />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
