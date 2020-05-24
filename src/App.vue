<template>
  <v-app>
    <Navigation />

    <v-content>
      <v-container>
        <v-row>
          <v-col cols="3">
          <About />
          </v-col>
        </v-row>
      </v-container>
    </v-content>
  </v-app>
</template>

<script>
import Navigation from './components/Navigation';
import About from './components/About'
export default {
  name: "App",

  components: {
    Navigation,
    About,
  },

  data: () => ({
    //
  })
};
</script>
