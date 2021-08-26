; -*- mode: Lisp -*-
(
(rust-mode .
             (
              (eval . (progn (add-hook 'before-save-hook #'lsp-format-buffer t t)
                             (add-hook 'before-save-hook #'lsp-organize-imports t t)
                       ))
)))
