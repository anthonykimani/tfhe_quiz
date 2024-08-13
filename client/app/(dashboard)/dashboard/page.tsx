"use client";

import { Button } from "@/components/ui/button";
import { quiz } from "@/helpers/quizSource";
import React, { useState } from "react";
import { io } from "socket.io-client";

const QuizPage = () => {
  const socket = io("http://0.0.0.0:8000");
  const [activeQuestion, setActiveQuestion] = useState(0);
  const [selectedAnswer, setSelectedAnswer] = useState(false);
  const [checked, setChecked] = useState(false);
  const [selectedAnswerIndex, setSelectedAnswerIndex] = useState(null);
  const [showResult, setShowResult] = useState(false);
  const [result, setResult] = useState({
    score: 0,
    correctAnswers: 0,
    wrongAnswers: 0,
  });

  const { questions } = quiz;
  const { question, answers, correctAnswer } = questions[activeQuestion];

  // socket.on("auth", () => {
  //   console.log("Starting Quiz"); // world
  // });

  // socket.emit("message");

  const onAnswerSelected = (answer: any, index: any) => {
    setChecked(true);
    setSelectedAnswerIndex(index);
    if (answer === correctAnswer) {
      setSelectedAnswer(true);
      console.log("true");
    } else {
      setSelectedAnswer(false);
      console.log("false");
    }
    setSelectedAnswerIndex(null);
    setResult((prev) =>
      selectedAnswer
        ? {
            ...prev,
            score: prev.score + 5,
            correctAnswers: prev.correctAnswers + 1,
          }
        : {
            ...prev,
            wrongAnswers: prev.wrongAnswers + 1,
          }
    );
    if (activeQuestion !== questions.length - 1) {
      setActiveQuestion((prev) => prev + 1);
    } else {
      setActiveQuestion(0);
      setShowResult(true);
    }
    setChecked(false);
  };

  const handleNextQuestion = () => {
    setSelectedAnswerIndex(null);
    setResult((prev) =>
      selectedAnswer
        ? {
            ...prev,
            score: prev.score + 5,
            correctAnswers: prev.correctAnswers + 1,
          }
        : {
            ...prev,
            wrongAnswers: prev.wrongAnswers + 1,
          }
    );
    if (activeQuestion !== questions.length - 1) {
      setActiveQuestion((prev) => prev + 1);
    } else {
      setActiveQuestion(0);
      setShowResult(true);
    }
    setChecked(false);
  };

  return (
    <section>
      <h1>
        Question: {activeQuestion + 1}/{questions.length}
      </h1>
      <div>
        {!showResult ? (
          <div>
            <div>{question}</div>
            <div className="flex flex-col">
              {answers.map((answer, index) => (
                <div key={index}>
                  <Button
                    variant={
                      selectedAnswerIndex === index ? "destructive" : "outline"
                    }
                    size={"fill"}
                    onClick={() => onAnswerSelected(answer, index)}
                  >
                    {answer}
                  </Button>
                </div>
              ))}
              {checked ? (
                <Button
                  variant={"default"}
                  size={"fill"}
                  onClick={() => handleNextQuestion()}
                >
                  {activeQuestion === questions.length - 1 ? "Finish" : "Next"}
                </Button>
              ) : (
                <Button
                  variant={"default"}
                  size={"fill"}
                  onClick={() => handleNextQuestion()}
                >
                  Skip Question
                </Button>
              )}
            </div>
          </div>
        ) : (
          <div>
            <h3>Results</h3>
            <h3>Overall {(result.score / 25) * 100}%</h3>
            <h4>
              Total Questions: <span>{questions.length}</span>
            </h4>
            <h4>
              Total Score: <span>{result.score}</span>
            </h4>
            <h4>
              Correct Answers: <span>{result.correctAnswers}</span>
            </h4>
            <h4>
              Wrong Answers: <span>{result.wrongAnswers}</span>
            </h4>
            <Button
              variant={"outline"}
              onClick={() => window.location.reload()}
            >
              Restart
            </Button>
          </div>
        )}
      </div>
    </section>
  );
};

export default QuizPage;
