use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_1_x_1_gb_sct (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-gb-sct" viewBox = "0 0 512 512" > < path fill = "#0065bd" d = "M0 0h512v512H0z" /> { "
  " } < path stroke = "#fff" stroke - width = ".6" d = "m0 0 5 3M0 3l5-3" transform = "scale(102.4 170.66667)" /> </ svg > } }