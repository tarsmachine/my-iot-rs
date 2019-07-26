// Shared navigation bar.
markup::define! {
    NavBar {
        nav.navbar[role = "navigation", "aria-label" = "main navigation"] {
            div.container {
                div."navbar-brand" {
                    a."navbar-burger".burger[
                        role = "button",
                        "aria-label" = "menu",
                        "aria-expanded" = "false",
                        "data-target" = "navbar-menu",
                    ] {
                        span["aria-hidden" = "true"] {}
                        span["aria-hidden" = "true"] {}
                        span["aria-hidden" = "true"] {}
                    }
                }

                div#"navbar-menu"."navbar-menu" {
                    div."navbar-start" {
                        a."navbar-item"[href = "/"] {
                            span.icon { i.fas."fa-home" {} }
                            span { "Home" }
                        }
                        a."navbar-item"[href = "/services"] {
                            span.icon { i.fas."fa-cog" {} }
                            span { "Services" }
                        }
                        a."navbar-item"[href = "/sensors"] {
                            span.icon { i.fas."fa-chart-line" {} }
                            span { "Sensors" }
                        }
                        a."navbar-item"[href = "/measurements"] {
                            span.icon { i.fas."fa-ruler-combined" {} }
                            span { "Measurements" }
                        }
                    }

                    div."navbar-end" {
                        a."navbar-item"[href = "https://eigenein.github.io/my-iot-rs/my_iot/"] {
                            span.icon { i.fas."fa-external-link-alt" {} }
                            span { "Docs" }
                        }
                    }
                }
            }
        }
    }
}