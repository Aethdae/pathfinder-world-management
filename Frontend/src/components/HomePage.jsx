import React, { useEffect, useState } from "react";
import { MAIN_URL } from "../helpers/consts";
import { homePageCardClasses } from "../helpers/htmlClasses";
import { Link } from "react-router";

export default function HomePage() {
  async function pingBackend() {
    fetch(MAIN_URL);
  }
  useEffect(() => {
    pingBackend();
  }, []);
  return (
    <div className="flex flex-col min-h-[90vh] gap-10 z-10 justify-around items-center bg-black w-[80vw] [border-style:groove] mx-auto border-4 border-white">
      <section
        className={
          homePageCardClasses.join(" ") + " min-h-[60vh] bg-red-400/30"
        }
      >
        <h2 className="text-4xl bg-black/70 w-full text-center py-6">
          Managing a world in any TTRPG is a difficult and arduous task.
        </h2>
        <p className="text-4xl bg-black/70 w-full text-center py-6">
          Keeping track of all the locations is a large burden on the mind.
        </p>
        <p className="text-4xl bg-black/70 w-full text-center py-6">
          This small app alleviates part of that information overload, making
          the world be easy to navigate through.
        </p>
      </section>

      <section
        className={homePageCardClasses.join(" ") + " min-h-[40vh]"}
      ></section>

      <section className={homePageCardClasses.join(" ") + " min-h-[60vh]"}>
        <div className="bg-gray-700 shadow-white shadow-lg/40 px-20 py-20 flex flex-col gap-10 rounded-xl rounded-bl-4xl rounded-tr-4xl">
          <h2 className="text-4xl underline-offset-8 underline text-center">
            View created resources
          </h2>
          <div className="flex gap-10"></div>
        </div>
        <div className="bg-gray-700 shadow-white shadow-lg/40 px-40 py-20 flex flex-col gap-10 rounded-xl rounded-bl-4xl rounded-tr-4xl">
          <h2 className="text-4xl underline-offset-8 underline text-center">
            Create more entries
          </h2>
        </div>
      </section>
    </div>
  );
}
