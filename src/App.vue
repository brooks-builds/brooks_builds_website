<template>
  <v-app>
    <Navigation />

    <v-content>
      <v-container>
        <v-row>
          <v-col cols="4">
            <v-row>
              <v-col>
                <About />
              </v-col>
            </v-row>
            <v-row>
              <v-col>
                <Schedule />
              </v-col>
            </v-row>
          </v-col>
          <v-spacer></v-spacer>
          <v-col cols="6" class="mt-7">
            <VideoCard />
          </v-col>
        </v-row>
      </v-container>
    </v-content>
  </v-app>
</template>

<script>
import Navigation from "./components/Navigation";
import About from "./components/About";
import Schedule from "./components/Schedule";
import VideoCard from "./components/VideoCard";

export default {
  name: "App",

  components: {
    Navigation,
    About,
    Schedule,
    VideoCard
  },

  data: () => ({
    //
  })
};
</script>
