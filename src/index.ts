import { serve } from "https://deno.land/std@0.156.0/http/server.ts";
import { handler } from "../build/mooretech_shortener.js";

serve(handler);
