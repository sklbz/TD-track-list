#!/bin/sh
bunx tailwindcss -i style/tailwind.css -o style/output.css --watch &
trunk serve
