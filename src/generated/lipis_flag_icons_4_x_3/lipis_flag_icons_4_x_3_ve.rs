use yew :: { Callback , Html , MouseEvent } ; # [inline (never)] pub fn lipis_flag_icons_4_x_3_ve (width : String , height : String , onclick : Option < Callback < MouseEvent >>) -> Html { yew :: html ! { < svg xmlns = "http://www.w3.org/2000/svg" data - license = "From https://github.com/lipis/flag-icons - Licensed under MIT" { width } { height } { onclick } fill = "currentColor" id = "flag-icons-ve" viewBox = "0 0 640 480" > < defs > { "
    " } < g id = "d" transform = "translate(0 -36)" > { "
      " } < g id = "c" > { "
        " } < g id = "b" > { "
          " } < path id = "a" fill = "#fff" d = "M0-5-1.5-.2l2.8.9z" /> { "
          " } < use href = "#a" width = "180" height = "120" transform = "scale(-1 1)" /> { "
        " } </ g > { "
        " } < use href = "#b" width = "180" height = "120" transform = "rotate(72)" /> { "
      " } </ g > { "
      " } < use href = "#b" width = "180" height = "120" transform = "rotate(-72)" /> { "
      " } < use href = "#c" width = "180" height = "120" transform = "rotate(144)" /> { "
    " } </ g > { "
  " } </ defs > { "
  " } < path fill = "#cf142b" d = "M0 0h640v480H0z" /> { "
  " } < path fill = "#00247d" d = "M0 0h640v320H0z" /> { "
  " } < path fill = "#fc0" d = "M0 0h640v160H0z" /> { "
  " } < g id = "f" transform = "matrix(4 0 0 4 320 336)" > { "
    " } < g id = "e" > { "
      " } < use href = "#d" width = "180" height = "120" transform = "rotate(10)" /> { "
      " } < use href = "#d" width = "180" height = "120" transform = "rotate(30)" /> { "
    " } </ g > { "
    " } < use href = "#e" width = "180" height = "120" transform = "rotate(40)" /> { "
  " } </ g > { "
  " } < use href = "#f" width = "180" height = "120" transform = "rotate(-80 320 336)" /> </ svg > } }