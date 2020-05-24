<template>
  <v-app>
    <Navigation />

    <v-content>
      <v-container>
        <v-row>
          <v-col cols="4">
          <About />
          </v-col>
        </v-row>
        <v-row>
          <v-col cols="4">
            <Schedule />
          </v-col>
        </v-row>
      </v-container>
    </v-content>
  </v-app>
</template>

<script>
import Navigation from './components/Navigation';
import About from './components/About';
import Schedule from './components/Schedule';

export default {
  name: "App",

  components: {
    Navigation,
    About,
    Schedule,
  },

  data: () => ({
    //
  })
};
</script>
