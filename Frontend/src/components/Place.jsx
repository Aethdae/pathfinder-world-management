import React, { useEffect, useState } from "react";
import { useParams } from "react-router";
// import CityCard from "../cards/CityCard";

import { MAIN_URL } from "../helpers/consts";

export default function Place() {
  const params = useParams();
  const [content, setContent] = useState({});
  async function getData() {
    try {
      console.log();
      const res = await fetch(MAIN_URL + `/${params.place}`);
      if (!res.ok) {
        throw new Error("Failed to fetch.");
      }
      const { name, data } = await res.json();
      console.log(name, data);
      if (data) {
        setContent(data);
      }
    } catch (error) {
      console.error(error);
    }
  }
  useEffect(() => {
    getData();
  }, []);
  return <div>place</div>;
}
