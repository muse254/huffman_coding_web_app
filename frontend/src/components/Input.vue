<template>
  <div>
    <div id="logo">
      <img src="../assets/logo.png" />
    </div>
    <form action="">
      <!--text to compress -->
      <label for="text">Text to compress:</label>

      <br />
      <textarea name="text" rows="4" cols="50">
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