use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_kw (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-kw" viewBox = "0 0 512 512" > < defs > { "
    " } < clipPath id = "kw-a" > { "
      " } < path fill - opacity = ".7" d = "M0 0h496v496H0z" /> { "
    " } </ clipPath > { "
  " } </ defs > { "
  " } < g fill - rule = "evenodd" stroke - width = "1pt" clip - path = "url(#kw-a)" transform = "scale(1.0321)" > { "
    " } < path fill = "#fff" d = "M0 165.3h992.1v165.4H0z" /> { "
    " } < path fill = "#f31830" d = "M0 330.7h992.1v165.4H0z" /> { "
    " } < path fill = "#00d941" d = "M0 0h992.1v165.4H0z" /> { "
    " } < path d = "M0 0v496l247.5-165.3.5-165.5L0 0z" /> { "
  " } </ g > </ svg > } }