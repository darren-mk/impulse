use maud::{html, Markup};
use crate::components::{icons::slash, layouts::to_html};
use axum::{response::Html};

pub async fn handler() -> Html<String> {
    let body: Markup = body();
    Html(to_html(body).into_string()) }

fn body() -> Markup {
    html! {
        div class="bg-white" {
          div class="pt-6" {
            nav aria-label="Breadcrumb" {
              ol role="list" class="mx-auto flex max-w-2xl items-center 
                                    space-x-2 px-4 sm:px-6 lg:max-w-7xl lg:px-8" {
                li {
                  div class="flex items-center" {
                    a href="#" 
                      class="mr-2 text-sm font-medium text-gray-900" { "Men" }
                    (slash()) } }
                li {
                  div class="flex items-center" {
                    a href="#" 
                      class="mr-2 text-sm font-medium text-gray-900" { "Clothing" }
                    (slash()) } }
                li class="text-sm" {
                  a href="#" 
                    aria-current="page" 
                    class="font-medium text-gray-500 hover:text-gray-600" { "Basic Tee 6-Pack" } } } }

            // Image gallery
            div class="mx-auto mt-6 max-w-2xl sm:px-6 lg:grid lg:max-w-7xl lg:grid-cols-3 lg:gap-8 lg:px-8" {
              img src="https://tailwindcss.com/plus-assets/img/ecommerce-images/product-page-02-secondary-product-shot.jpg" alt="Two each of gray, white, and black shirts laying flat." class="row-span-2 aspect-3/4 size-full rounded-lg object-cover max-lg:hidden";
              img src="https://tailwindcss.com/plus-assets/img/ecommerce-images/product-page-02-tertiary-product-shot-01.jpg" alt="Model wearing plain black basic tee." class="col-start-2 aspect-3/2 size-full rounded-lg object-cover max-lg:hidden";
              img src="https://tailwindcss.com/plus-assets/img/ecommerce-images/product-page-02-tertiary-product-shot-02.jpg" alt="Model wearing plain gray basic tee." class="col-start-2 row-start-2 aspect-3/2 size-full rounded-lg object-cover max-lg:hidden";
              img src="https://tailwindcss.com/plus-assets/img/ecommerce-images/product-page-02-featured-product-shot.jpg" alt="Model wearing plain white basic tee." class="row-span-2 aspect-4/5 size-full object-cover sm:rounded-lg lg:aspect-3/4";
            }

            // Product info
            div class="mx-auto max-w-2xl px-4 pt-10 pb-16 sm:px-6 lg:grid lg:max-w-7xl lg:grid-cols-3 lg:grid-rows-[auto_auto_1fr] lg:gap-x-8 lg:px-8 lg:pt-16 lg:pb-24" {
              div class="lg:col-span-2 lg:border-r lg:border-gray-200 lg:pr-8" {
                h1 class="text-2xl font-bold tracking-tight text-gray-900 sm:text-3xl" { "Basic Tee 6-Pack" }
              }

              // Options
              div class="mt-4 lg:row-span-3 lg:mt-0" {
                h2 class="sr-only" { "Product information" }
                p class="text-3xl tracking-tight text-gray-900" { "$192" }

                // Reviews
                div class="mt-6" {
                  h3 class="sr-only" { "Reviews" }
                  div class="flex items-center" {
                    div class="flex items-center" {
                      // Active stars
                      svg viewBox="0 0 20 20" fill="currentColor" data-slot="icon" aria-hidden="true" class="size-5 shrink-0 text-gray-900" {
                        path d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401Z" clip-rule="evenodd" fill-rule="evenodd" {};
                      }
                      svg viewBox="0 0 20 20" fill="currentColor" data-slot="icon" aria-hidden="true" class="size-5 shrink-0 text-gray-900" {
                        path d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401Z" clip-rule="evenodd" fill-rule="evenodd" {};
                      }
                      svg viewBox="0 0 20 20" fill="currentColor" data-slot="icon" aria-hidden="true" class="size-5 shrink-0 text-gray-900" {
                        path d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401Z" clip-rule="evenodd" fill-rule="evenodd" {};
                      }
                      svg viewBox="0 0 20 20" fill="currentColor" data-slot="icon" aria-hidden="true" class="size-5 shrink-0 text-gray-900" {
                        path d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401Z" clip-rule="evenodd" fill-rule="evenodd" {};
                      }
                      svg viewBox="0 0 20 20" fill="currentColor" data-slot="icon" aria-hidden="true" class="size-5 shrink-0 text-gray-200" {
                        path d="M10.868 2.884c-.321-.772-1.415-.772-1.736 0l-1.83 4.401-4.753.381c-.833.067-1.171 1.107-.536 1.651l3.62 3.102-1.106 4.637c-.194.813.691 1.456 1.405 1.02L10 15.591l4.069 2.485c.713.436 1.598-.207 1.404-1.02l-1.106-4.637 3.62-3.102c.635-.544.297-1.584-.536-1.65l-4.752-.382-1.831-4.401Z" clip-rule="evenodd" fill-rule="evenodd" {};
                      }
                    }
                    p class="sr-only" { "4 out of 5 stars" }
                    a href="#" class="ml-3 text-sm font-medium text-indigo-600 hover:text-indigo-500" { "117 reviews" }
                  }
                }

                form class="mt-10" {
                  // Colors
                  div {
                    h3 class="text-sm font-medium text-gray-900" { "Color" }

                    fieldset aria-label="Choose a color" class="mt-4" {
                      div class="flex items-center gap-x-3" {
                        div class="flex rounded-full outline -outline-offset-1 outline-black/10" {
                          input type="radio" name="color" value="white" checked="checked" aria-label="White" class="size-8 appearance-none rounded-full bg-white forced-color-adjust-none checked:outline-2 checked:outline-offset-2 checked:outline-gray-400 focus-visible:outline-3 focus-visible:outline-offset-3";
                        }
                        div class="flex rounded-full outline -outline-offset-1 outline-black/10" {
                          input type="radio" name="color" value="gray" aria-label="Gray" class="size-8 appearance-none rounded-full bg-gray-200 forced-color-adjust-none checked:outline-2 checked:outline-offset-2 checked:outline-gray-400 focus-visible:outline-3 focus-visible:outline-offset-3";
                        }
                        div class="flex rounded-full outline -outline-offset-1 outline-black/10" {
                          input type="radio" name="color" value="black" aria-label="Black" class="size-8 appearance-none rounded-full bg-gray-900 forced-color-adjust-none checked:outline-2 checked:outline-offset-2 checked:outline-gray-900 focus-visible:outline-3 focus-visible:outline-offset-3";
                        }
                      }
                    }
                  }

                  // Sizes
                  div class="mt-10" {
                    div class="flex items-center justify-between" {
                      h3 class="text-sm font-medium text-gray-900" { "Size" }
                      a href="#" class="text-sm font-medium text-indigo-600 hover:text-indigo-500" { "Size guide" }
                    }

                    fieldset aria-label="Choose a size" class="mt-4" {
                      div class="grid grid-cols-4 gap-3" {
                        label aria-label="XXS" class="group relative flex items-center justify-center rounded-md border border-gray-300 bg-white p-3 has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25" {
                          input type="radio" name="size" disabled="disabled" class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed";
                          span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white" { "XXS" }
                        }
                        label aria-label="XS" class="group relative flex items-center justify-center rounded-md border border-gray-300 bg-white p-3 has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25" {
                          input type="radio" name="size" class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed";
                          span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white" { "XS" }
                        }
                        label aria-label="S" class="group relative flex items-center justify-center rounded-md border border-gray-300 bg-white p-3 has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25" {
                          input type="radio" name="size" checked="checked" class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed";
                          span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white" { "S" }
                        }
                        label aria-label="M" class="group relative flex items-center justify-center rounded-md border border-gray-300 bg-white p-3 has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25" {
                          input type="radio" name="size" class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed";
                          span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white" { "M" }
                        }
                        label aria-label="L" class="group relative flex items-center justify-center rounded-md border border-gray-300 bg-white p-3 has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25" {
                          input type="radio" name="size" class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed";
                          span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white" { "L" }
                        }
                        label aria-label="XL" class="group relative flex items-center justify-center rounded-md border border-gray-300 bg-white p-3 has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25" {
                          input type="radio" name="size" class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed";
                          span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white" { "XL" }
                        }
                        label aria-label="2XL" class="group relative flex items-center justify-center rounded-md border border-gray-300 bg-white p-3 has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25" {
                          input type="radio" name="size" class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed";
                          span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white" { "2XL" }
                        }
                        label aria-label="3XL" class="group relative flex items-center justify-center rounded-md border border-gray-300 bg-white p-3 has-checked:border-indigo-600 has-checked:bg-indigo-600 has-focus-visible:outline-2 has-focus-visible:outline-offset-2 has-focus-visible:outline-indigo-600 has-disabled:border-gray-400 has-disabled:bg-gray-200 has-disabled:opacity-25" {
                          input type="radio" name="size" class="absolute inset-0 appearance-none focus:outline-none disabled:cursor-not-allowed";
                          span class="text-sm font-medium text-gray-900 uppercase group-has-checked:text-white" { "3XL" }
                        }
                      }
                    }
                  }

                  button type="submit" class="mt-10 flex w-full items-center justify-center rounded-md border border-transparent bg-indigo-600 px-8 py-3 text-base font-medium text-white hover:bg-indigo-700 focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2 focus:outline-hidden" { "Add to bag" }
                }
              }

              div class="py-10 lg:col-span-2 lg:col-start-1 lg:border-r lg:border-gray-200 lg:pt-6 lg:pr-8 lg:pb-16" {
                // Description and details
                div {
                  h3 class="sr-only" { "Description" }
                  div class="space-y-6" {
                    p class="text-base text-gray-900" {
                      "The Basic Tee 6-Pack allows you to fully express your vibrant personality with three grayscale options. Feeling adventurous? Put on a heather gray tee. Want to be a trendsetter? Try our exclusive colorway: &quot;Black&quot;. Need to add an extra pop of color to your outfit? Our white tee has you covered."
                    }
                  }
                }

                div class="mt-10" {
                  h3 class="text-sm font-medium text-gray-900" { "Highlights" }
                  div class="mt-4" {
                    ul role="list" class="list-disc space-y-2 pl-4 text-sm" {
                      li class="text-gray-400" { span class="text-gray-600" { "Hand cut and sewn locally" } }
                      li class="text-gray-400" { span class="text-gray-600" { "Dyed with our proprietary colors" } }
                      li class="text-gray-400" { span class="text-gray-600" { "Pre-washed &amp; pre-shrunk" } }
                      li class="text-gray-400" { span class="text-gray-600" { "Ultra-soft 100% cotton" } }
                    }
                  }
                }

                div class="mt-10" {
                  h2 class="text-sm font-medium text-gray-900" { "Details" }
                  div class="mt-4 space-y-6" {
                    p class="text-sm text-gray-600" {
                      "The 6-Pack includes two black, two white, and two heather gray Basic Tees. Sign up for our subscription service and be the first to get new, exciting colors, like our upcoming &quot;Charcoal Gray&quot; limited release."
                    }
                  }
                }
              }
            }
          }
        }
    }
}
