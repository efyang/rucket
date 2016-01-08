#lang racket/base

(provide get_board_padding get_country)

(require ffi/unsafe
         ffi/unsafe/define
         racket/path)

(define cd (path-only (path->complete-path (find-system-path 'run-file))))
(define libname "librmapcollision")
(define libpath (string-append (path->string cd) libname))
(define-ffi-definer define-rust (ffi-lib libpath))

(define-rust get_x_pad (_fun _int -> _int))
(define-rust get_y_pad (_fun _int -> _int))
(define (get_board_padding screenwidth screenheight)
  (values (get_x_pad screenwidth) (get_y_pad screenheight)))

; (get_country mousex mousey board_x_pad board_y_pad) -> country name
(define-rust get_country (_fun _int _int _int _int -> _string))
