#!/bin/bash

#start the alinvoinea_api function first with cargo lambda watch command
cargo lambda invoke --data-ascii "{ \"action\": \"Query\", \"query\": \"query GetCategories {categories {name slug}}\"  }"