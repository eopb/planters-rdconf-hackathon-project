use seed_style::*;

pub fn global_init() {
    // make everything border-box
    GlobalStyle::default()
        .style(
            "a,ul,li,div,p,h1,h2,h3,h4,li,dd,dt,button,label,input",
            s().font_family("'Lato',sans-serif")
                .webkit_font_smoothing_antialiased(),
        )
        .style("body", s().box_sizing_border_box())
        .style("img", s().box_sizing_content_box())
        .style("*, *:before, *:after", s().box_sizing("inherit"))
        .activate_init_styles();
}
