use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_be (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-be" viewBox = "0 0 640 480" > < g fill - rule = "evenodd" stroke - width = "1pt" > { "
    " } < path d = "M0 0h213.3v480H0z" /> { "
    " } < path fill = "#ffd90c" d = "M213.3 0h213.4v480H213.3z" /> { "
    " } < path fill = "#f31830" d = "M426.7 0H640v480H426.7z" /> { "
  " } </ g > </ svg > } }