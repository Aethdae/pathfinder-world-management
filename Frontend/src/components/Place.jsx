import React, { useEffect, useState } from "react";
import { useParams } from "react-router";
// import CityCard from "../cards/CityCard";

import { MAIN_URL } from "../helpers/consts";

export default function Place() {
  const params = useParams();
  const [content, setContent] = useState({});
  async function getData() {
    try {
      const res = await fetch(MAIN_URL + `/${params.city}`);
      if (!res.ok) {
        throw new Error("Failed to fetch.");
      }
      const { name, data } = await res.json();
      if (data) {
        setContent(data);
      }
    } catch (error) {
      console.error(error);
    }
  }
  return <div>place</div>;
}
