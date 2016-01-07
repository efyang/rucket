#lang racket/base

(provide get_country)

(require ffi/unsafe
         ffi/unsafe/define
         racket/path)

(define cd (path-only (path->complete-path (find-system-path 'run-file))))
(define libname "librmapcollision")
(define libpath (string-append (path->string cd) libname))
(define-values (map-default-width map-default-height) (1306.8 875.4))
(define-ffi-definer define-rust (ffi-lib libpath))

(define-rust print_parsed (_fun -> _int))
(define-rust get_country (_fun _int _int _int _int -> _string))

(print_parsed)
(print (get_country 1 2 3 4))
