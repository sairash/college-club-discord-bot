<script setup lang="ts">
import axios from "axios"
import { onMounted, ref } from 'vue';


function b(input: string): string {
  let return_text: string = "";
  for (let i = 0; i < input.length; i++) {
    return_text += (input.charCodeAt(i) ^ 5).toString(16);
  }
  return return_text;
}

const hasCodeParam = ref(false);
const text = ref("abc");
const error = ref("");
var sending_request = false;
var codeValue = ""


onMounted(() => {
  const searchParams = new URLSearchParams(window.location.search);
  if (searchParams.has('code')) {
    hasCodeParam.value = true;
    codeValue = searchParams.get('code')!;
    const newUrl = '/';
    window.history.pushState({ path: newUrl }, '', newUrl);
  }
})

function login_with_discord() {
  if (!hasCodeParam.value) {
    location.replace(import.meta.env.VITE_URL)
  } else {
    alert("You are Already Logged in!")
  }
}


async function attendance() {
  console.log("Here")
  error.value = ""
  if(sending_request){
    return
  }
  sending_request = true;
  if ("geolocation" in navigator) {
    navigator.geolocation.getCurrentPosition(
      (position) => {
        const latitude = position.coords.latitude;
        const longitude = position.coords.longitude;
        alert(latitude)
        alert(longitude)
        if (latitude.toString().length < 9) {
          error.value = "Brave is not Support for this Operation!"
          sending_request = false;
          return
        }
        if (!hasCodeParam.value) {
          alert("Login with Discord!");
          sending_request = false;
          return
        }
        axios.post("/attendance", {
          "l": b(latitude.toString()) + "|" + b(longitude.toString()),
          "d": b(codeValue),
          "c": b(text.value),
        }, {
          headers: {
            'Content-Type': 'application/json'
          }
        })
      },
      () => {
        error.value = "Use Another Browser";
      }
    );
  } else {
    error.value = "Use Another Browser!";
  }
  sending_request = false;
}
</script>

<template>
  <section class="bg-white">
    <div class="lg:grid lg:min-h-screen lg:grid-cols-12">
      <section class="relative flex h-32 items-end bg-gray-900 lg:col-span-5 lg:h-full xl:col-span-6">
        <img alt=""
          src="https://images.unsplash.com/photo-1617195737496-bc30194e3a19?ixlib=rb-1.2.1&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=870&q=80"
          class="absolute inset-0 h-full w-full object-cover opacity-80" />

        <div class="hidden lg:relative lg:block lg:p-12">
          <div class="block text-white">
            <span class="sr-only">Home</span>
            <svg class="h-8 sm:h-10" viewBox="0 0 28 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path
                d="M0.41 10.3847C1.14777 7.4194 2.85643 4.7861 5.2639 2.90424C7.6714 1.02234 10.6393 0 13.695 0C16.7507 0 19.7186 1.02234 22.1261 2.90424C24.5336 4.7861 26.2422 7.4194 26.98 10.3847H25.78C23.7557 10.3549 21.7729 10.9599 20.11 12.1147C20.014 12.1842 19.9138 12.2477 19.81 12.3047H19.67C19.5662 12.2477 19.466 12.1842 19.37 12.1147C17.6924 10.9866 15.7166 10.3841 13.695 10.3841C11.6734 10.3841 9.6976 10.9866 8.02 12.1147C7.924 12.1842 7.8238 12.2477 7.72 12.3047H7.58C7.4762 12.2477 7.376 12.1842 7.28 12.1147C5.6171 10.9599 3.6343 10.3549 1.61 10.3847H0.41ZM23.62 16.6547C24.236 16.175 24.9995 15.924 25.78 15.9447H27.39V12.7347H25.78C24.4052 12.7181 23.0619 13.146 21.95 13.9547C21.3243 14.416 20.5674 14.6649 19.79 14.6649C19.0126 14.6649 18.2557 14.416 17.63 13.9547C16.4899 13.1611 15.1341 12.7356 13.745 12.7356C12.3559 12.7356 11.0001 13.1611 9.86 13.9547C9.2343 14.416 8.4774 14.6649 7.7 14.6649C6.9226 14.6649 6.1657 14.416 5.54 13.9547C4.4144 13.1356 3.0518 12.7072 1.66 12.7347H0V15.9447H1.61C2.39051 15.924 3.154 16.175 3.77 16.6547C4.908 17.4489 6.2623 17.8747 7.65 17.8747C9.0377 17.8747 10.392 17.4489 11.53 16.6547C12.1468 16.1765 12.9097 15.9257 13.69 15.9447C14.4708 15.9223 15.2348 16.1735 15.85 16.6547C16.9901 17.4484 18.3459 17.8738 19.735 17.8738C21.1241 17.8738 22.4799 17.4484 23.62 16.6547ZM23.62 22.3947C24.236 21.915 24.9995 21.664 25.78 21.6847H27.39V18.4747H25.78C24.4052 18.4581 23.0619 18.886 21.95 19.6947C21.3243 20.156 20.5674 20.4049 19.79 20.4049C19.0126 20.4049 18.2557 20.156 17.63 19.6947C16.4899 18.9011 15.1341 18.4757 13.745 18.4757C12.3559 18.4757 11.0001 18.9011 9.86 19.6947C9.2343 20.156 8.4774 20.4049 7.7 20.4049C6.9226 20.4049 6.1657 20.156 5.54 19.6947C4.4144 18.8757 3.0518 18.4472 1.66 18.4747H0V21.6847H1.61C2.39051 21.664 3.154 21.915 3.77 22.3947C4.908 23.1889 6.2623 23.6147 7.65 23.6147C9.0377 23.6147 10.392 23.1889 11.53 22.3947C12.1468 21.9165 12.9097 21.6657 13.69 21.6847C14.4708 21.6623 15.2348 21.9135 15.85 22.3947C16.9901 23.1884 18.3459 23.6138 19.735 23.6138C21.1241 23.6138 22.4799 23.1884 23.62 22.3947Z"
                fill="currentColor" />
            </svg>
          </div>

          <h2 class="mt-6 text-2xl font-bold text-white sm:text-3xl md:text-4xl">
            Web Dev Attendance
          </h2>

          <p class="mt-4 leading-relaxed text-white/90">
            This is an Attendance for Web & App Dev Club for Discord: <br>Made by <a target="_blank"
              class="text-blue-500 border-b-2 border-blue-500"
              href="https://www.linkedin.com/in/sairash-gautam-54b743239/">Sairash</a>
          </p>
        </div>
      </section>

      <main class="flex items-center justify-center px-8 py-8 sm:px-12 lg:col-span-7 lg:px-16 lg:py-12 xl:col-span-6">
        <div class="max-w-xl lg:max-w-3xl">
          <div class="relative -mt-16 block lg:hidden">
            <div class="inline-flex size-16 items-center justify-center rounded-full bg-white text-blue-600 sm:size-20">
              <span class="sr-only">Home</span>
              <svg class="h-8 sm:h-10" viewBox="0 0 28 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                <path
                  d="M0.41 10.3847C1.14777 7.4194 2.85643 4.7861 5.2639 2.90424C7.6714 1.02234 10.6393 0 13.695 0C16.7507 0 19.7186 1.02234 22.1261 2.90424C24.5336 4.7861 26.2422 7.4194 26.98 10.3847H25.78C23.7557 10.3549 21.7729 10.9599 20.11 12.1147C20.014 12.1842 19.9138 12.2477 19.81 12.3047H19.67C19.5662 12.2477 19.466 12.1842 19.37 12.1147C17.6924 10.9866 15.7166 10.3841 13.695 10.3841C11.6734 10.3841 9.6976 10.9866 8.02 12.1147C7.924 12.1842 7.8238 12.2477 7.72 12.3047H7.58C7.4762 12.2477 7.376 12.1842 7.28 12.1147C5.6171 10.9599 3.6343 10.3549 1.61 10.3847H0.41ZM23.62 16.6547C24.236 16.175 24.9995 15.924 25.78 15.9447H27.39V12.7347H25.78C24.4052 12.7181 23.0619 13.146 21.95 13.9547C21.3243 14.416 20.5674 14.6649 19.79 14.6649C19.0126 14.6649 18.2557 14.416 17.63 13.9547C16.4899 13.1611 15.1341 12.7356 13.745 12.7356C12.3559 12.7356 11.0001 13.1611 9.86 13.9547C9.2343 14.416 8.4774 14.6649 7.7 14.6649C6.9226 14.6649 6.1657 14.416 5.54 13.9547C4.4144 13.1356 3.0518 12.7072 1.66 12.7347H0V15.9447H1.61C2.39051 15.924 3.154 16.175 3.77 16.6547C4.908 17.4489 6.2623 17.8747 7.65 17.8747C9.0377 17.8747 10.392 17.4489 11.53 16.6547C12.1468 16.1765 12.9097 15.9257 13.69 15.9447C14.4708 15.9223 15.2348 16.1735 15.85 16.6547C16.9901 17.4484 18.3459 17.8738 19.735 17.8738C21.1241 17.8738 22.4799 17.4484 23.62 16.6547ZM23.62 22.3947C24.236 21.915 24.9995 21.664 25.78 21.6847H27.39V18.4747H25.78C24.4052 18.4581 23.0619 18.886 21.95 19.6947C21.3243 20.156 20.5674 20.4049 19.79 20.4049C19.0126 20.4049 18.2557 20.156 17.63 19.6947C16.4899 18.9011 15.1341 18.4757 13.745 18.4757C12.3559 18.4757 11.0001 18.9011 9.86 19.6947C9.2343 20.156 8.4774 20.4049 7.7 20.4049C6.9226 20.4049 6.1657 20.156 5.54 19.6947C4.4144 18.8757 3.0518 18.4472 1.66 18.4747H0V21.6847H1.61C2.39051 21.664 3.154 21.915 3.77 22.3947C4.908 23.1889 6.2623 23.6147 7.65 23.6147C9.0377 23.6147 10.392 23.1889 11.53 22.3947C12.1468 21.9165 12.9097 21.6657 13.69 21.6847C14.4708 21.6623 15.2348 21.9135 15.85 22.3947C16.9901 23.1884 18.3459 23.6138 19.735 23.6138C21.1241 23.6138 22.4799 23.1884 23.62 22.3947Z"
                  fill="currentColor" />
              </svg>
            </div>

            <h1 class="mt-2 text-2xl font-bold text-gray-900 sm:text-3xl md:text-4xl">
              Web Dev Attendance
            </h1>

            <p class="mt-4 leading-relaxed text-gray-500">
              This is an Attendance for Web & App Dev Club for Discord: <br>Made by <a target="_blank"
                class="text-blue-500 border-b-2 border-blue-500"
                href="https://www.linkedin.com/in/sairash-gautam-54b743239/">Sairash</a>
            </p>
          </div>

          <div class="mt-8 grid grid-cols-6 gap-6">
            <div class="col-span-6">
              <button @click="login_with_discord"
                class="px-4 w-full py-2 text-white border-0 rounded flex justify-center my-2"
                style="background-color: #7289da">
                <img style="margin-top: 4.5px;" class="w-5 mr-2"
                  src="https://assets-global.website-files.com/6257adef93867e50d84d30e2/653714c1f22aef3b6921d63d_636e0a6ca814282eca7172c6_icon_clyde_white_RGB.svg"
                  alt="">
                <span v-if="!hasCodeParam" class="font-bold">Login With Discord</span>
                <span v-else class="font-bold">Logged in</span>
              </button>
            </div>

            <div class="col-span-6">
              <input v-model="text"
                class="text-lg bg-gray-200 appearance-none border-2 border-gray-200 text-center rounded w-full py-2 px-4 text-gray-700 leading-tight h-12 focus:outline-none focus:bg-white focus:border-blue-700"
                placeholder="Enter Code" type="text">
            </div>

            <div class="col-span-6 " v-if="error != ''">
              <div
                class="text-xl text-center bg-rose-500 appearance-none  rounded py-2 px-4 text-white leading-tight rounded">
                {{ error }}
              </div>
            </div>

            <div class="col-span-6 mt-1">
              <button @click="attendance"
                class="inline-block w-full shrink-0 rounded-md border border-blue-600 bg-blue-600 px-12 py-3 text-sm font-medium text-white transition hover:bg-transparent hover:text-blue-600 focus:outline-none focus:ring active:text-blue-500">
                Present Today
              </button>
            </div>
          </div>
        </div>
      </main>
    </div>
  </section>
</template>
