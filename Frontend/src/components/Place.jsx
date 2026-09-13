import React, { useEffect, useState } from "react";
import { useParams } from "react-router";
// import CityCard from "../cards/CityCard";

import { MAIN_URL } from "../helpers/consts";

export default function Place() {
  const params = useParams();
  const [content, setContent] = useState({});
  async function getData() {
    try {
      const res = await fetch(MAIN_URL + `/${params.place}`);
      if (!res.ok) {
        throw new Error("Failed to fetch.");
      }
      const result = await res.json();
      console.log(result);
      console.log(result.result);
      console.log(result.result[0].results[0]);
      if (result.result[0].results[0].id) {
        setContent(result.result[0].results[0]);
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
