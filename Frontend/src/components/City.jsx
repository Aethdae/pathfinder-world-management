import React, { useEffect, useState } from "react";
import CityCard from "../cards/CityCard";
import { useParams } from "react-router";
import { MAIN_URL } from "../helpers/consts";

export default function City() {
  const params = useParams();
  const [city, setCity] = useState({});
  async function grabCity() {
    try {
      const res = await fetch(MAIN_URL + `/${params.city}`);
      if (!res.ok) {
        throw new Error("Failed to get city!");
      }
      const data = await res.json();
      if (data.city) {
        setCity(data.city);
      }
    } catch (error) {
      console.error(error);
    }
  }
  useEffect(() => {
    grabCity();
  }, []);
  return (
    <div className="flex flex-col min-h-[40vh] gap-10 z-10 justify-around items-center bg-black w-[80vw] [border-style:groove] mx-auto border-4 border-white">
      <div className="w-[80%] mx-auto py-10">
        {city.name?.length > 0 && <CityCard city={city} inDepth={true} />}
      </div>
    </div>
  );
}
