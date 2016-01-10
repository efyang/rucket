#lang racket

(provide set_screen_size 
	 get_current_board_padding
	 get_country_on_mouse
	 get_country_list)

(require ffi/unsafe
         ffi/unsafe/define
         racket/path)

(define CD (path-only (path->complete-path (find-system-path 'run-file))))
(define LIBNAME "librmapcollision")
(define LIBPATH (string-append (path->string CD) LIBNAME))
; define all FFI functions
(define-ffi-definer define-rust (ffi-lib LIBPATH))
(define-rust set_padding (_fun _int _int -> _int))
(define-rust get_x_pad (_fun -> _int))
(define-rust get_y_pad (_fun -> _int))
(define-rust get_country (_fun _int _int -> _string))
(define-rust get_countryliststr (_fun -> _string))

; Provided Definitions

; sets the screen size for the library
; screenwidth, screenheight -> void
(define (set_screen_size screenwidth screenheight)
  (set_padding screenwidth screenheight))
; gets the current board padding (how much the board 
; is offset from the origin)
; void -> (values xpad ypad)
(define (get_current_board_padding)
  (values (get_x_pad) (get_y_pad)))
; gets the country based on the mouse coordinates
; make sure that the screen size is set first with 
; set_screen_size before using this function
; mousex mousey -> countryname
(define (get_country_on_mouse mousex mousey)
  (get_country mousex mousey))
; gets the list of all countries
; void -> (list countryname)
(define (get_country_list)
  (string-split (get_countryliststr) "|"))


(define (tests)
  (set_screen_size 1920 1080)
  (define-values (xpad ypad) (get_current_board_padding))
  (println xpad)
  (println ypad)
  (println (get_country_list))
  )
(tests)
