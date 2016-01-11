#lang racket/gui
(require "rust_functions.rkt")
(require picturing-programs)

(define-values (screenw screenh)
  (get-display-size))
(define board (freeze (bitmap "scaledboard.png")))
(define boardwithbuttons (freeze (above board
                                        (overlay (text "Buttons" 36 "black")
                                                 (rectangle screenw 75 "solid" "gray")))))
(define basecanvas (freeze (rectangle screenw screenh "solid" "white")))
(define model (list 0 0))
(define (mousehandler model x y event)
  (list x y))
(define (render model)
  (local [(define x (list-ref model 0))
          (define y (list-ref model 1))
          (define displaytext (text (string-append "X: " 
                                                   (number->string x)
                                                   ", Y: "
                                                   (number->string y)
                                                   ", Country: "
                                                   (get_country_on_mouse x y)) 
                                    36
                                    "black"))]
    (overlay/align "center" 
                   "bottom"
                   displaytext
                   (overlay boardwithbuttons
                            basecanvas))))
(set_screen_size screenw screenh)
(define-values (xpad ypad) (get_current_board_padding))
(println xpad)
(println ypad)
(println (get_country_list))

(big-bang model
          (display-mode 'fullscreen)  
          (on-draw render)
          (on-mouse mousehandler))
