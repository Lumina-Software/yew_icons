use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn bootstrap_arrow_bar_left (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/twbs/icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" class = "bi bi-arrow-bar-left" viewBox = "0 0 16 16" > < path fill - rule = "evenodd" d = "M12.5 15a.5.5 0 0 1-.5-.5v-13a.5.5 0 0 1 1 0v13a.5.5 0 0 1-.5.5zM10 8a.5.5 0 0 1-.5.5H3.707l2.147 2.146a.5.5 0 0 1-.708.708l-3-3a.5.5 0 0 1 0-.708l3-3a.5.5 0 1 1 .708.708L3.707 7.5H9.5a.5.5 0 0 1 .5.5z" /> </ svg > } }