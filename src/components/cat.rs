use leptos::*;
use crate::models::cat::CatModel;

#[component]
pub fn Cat() -> impl IntoView {
    let cats = vec![
        CatModel {
            id: 1,
            name: "Lookhin".to_string(),
            image: "https://www.catster.com/wp-content/uploads/2023/11/Abyssinian-serious.jpg".to_string(),
            age: 10,
            breed: "Abyssinian".to_string(),
        },
        CatModel {
            id: 2,
            name: "Kaow".to_string(),
            image: "https://www.litter-robot.com/media/magefan_blog/singapura-cat8.png".to_string(),
            age: 1,
            breed: "Singapura".to_string(),
        },
    ];

    let (cat_list, set_cat_list) = create_signal(cats);

    view! {
        <div class="cat">
            <ul>
                <For
                    each=cat_list
                    key=|cat| cat.id
                    children=move |cat| {
                        view! {
                            <li class="cat-card">
                                <div><strong>"Name: "</strong> {cat.name}</div>
                                <div><strong>"Age: "</strong> {cat.age}</div>
                                <div><strong>"Breed: "</strong> {cat.breed}</div>
                                <img src={cat.image} />
                            </li>
                        }
                    }
                />
            </ul>
        </div>
    }
}