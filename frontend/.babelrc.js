"use strict";

const manifest = require("./package.json");

module.exports = {
    parserOpts: {
        strictMode: true,
    },
    plugins: [
        "relay",
        "@emotion",
    ],
    presets: [
        ["@babel/preset-env", {
            // Set to `true` to show which transforms will be run
            // during the build
            debug: false,
            targets: manifest.browserslist,
        }],
        [
            "@babel/preset-typescript",
            {
                "allowDeclareFields": true,
            },
        ],
        [
            "@babel/preset-react",
            {
                "runtime": "automatic",
                "importSource": "@emotion/react",
            },
        ],
    ],
};
