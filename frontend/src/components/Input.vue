<template>
  <div>
    <div id="logo">
      <img src="../assets/logo.png" />
    </div>
    <form @submit.prevent="compressText">
      <!--text to compress -->
      <label for="text">Text to compress:</label>

      <br />
      <textarea required v-model="text" rows="4" cols="50">
		Lorem ipsum dolor sit amet consectetur adipisicing elit. Odio, fuga! Minima aperiam distinctio enim et excepturi laborum, mollitia cupiditate, error, iusto neque blanditiis. Unde magnam libero quod ipsum eveniet ex.
	</textarea
      >

      <br />
      <!-- submit button -->
      <button>Compress</button>
    </form>
  </div>
</template>

<script>
export default {
  name: "Input",

  data() {
    return {
      text: "",
    };
  },

  methods: {
    compressText() {
      // send data to server
      const request_options = {
        method: "POST",
        credentials: "include",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          text: this.text,
        }),
      };

      fetch("http://localhost:8000/compress", request_options)
        .then(async (response) => {
          const data = await response.json();

          if (!response.ok) {
            // get error message from body or default to response status
            const error = (data && data.message) || response.status;
            return Promise.reject(error);
          }
          
          // emit event
          this.$emit("compressed-data", data);
        })
        .catch((error) => {
          console.error("There was an error!", error);
          alert(`${error}`);
        });
    },
  },
};
</script>

<style scoped>
form {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}

#logo {
  width: 100%;
  text-align: center;
  padding: 20px;
}

#logo img {
  width: 150px;
  height: 150px;
}
</style>