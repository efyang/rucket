#lang racket/base

(provide get_country)

(require ffi/unsafe
         ffi/unsafe/define
         racket/path)

(define cd (path-only (path->complete-path (find-system-path 'run-file))))
(define libname "librmapcollision")
(define libpath (string-append (path->string cd) libname))
(define-ffi-definer define-rust (ffi-lib libpath))

(define-rust print_countries (_fun -> _int))
(define-rust get_country (_fun _int _int -> _string))

(print_countries)
(println (get_country 35 75))

