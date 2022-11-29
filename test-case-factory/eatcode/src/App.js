import { colors } from './global/vars'
import './global/fonts.css';
import { userContext } from './userContext';
import Axios from "axios"; //keep for when we verify beef and admin ideally

import { Component } from "react";
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import HeaderSkip from './components/common/HeaderSkip';
import Landing from './pages/landing';
import Problems from './pages/problems';
import Question from './pages/question';
import Create from './pages/create';
import User from './pages/user'
import Error from './pages/error';

class App extends Component {
  loggedOutUserObject = {
    userName: "Not Logged In",
    userID: null,
    email: null,
    userProfilePictureUrl: "https://cdn.pixabay.com/photo/2017/02/12/21/29/false-2061132_1280.png",
    isAdmin: false,
    totalScore: 0,
    attemptedProblems: new Map([])
  }

  constructor(props) {
    super(props)

    this.state = {
      user: {
        userName: this.loggedOutUserObject.userName,
        userID: this.loggedOutUserObject.userID,
        email: this.loggedOutUserObject.email,
        userProfilePictureUrl: this.loggedOutUserObject.userProfilePictureUrl,
        isAdmin: this.loggedOutUserObject.isAdmin,
        totalScore: this.loggedOutUserObject.totalScore,
        attemptedProblems: this.loggedOutUserObject.attemptedProblems
      }
    }

    this.logIn = this.logIn.bind(this)
    this.logOut = this.logOut.bind(this)


  }

  componentDidMount() {
    const loggedInUser = localStorage.getItem("user");
    if (loggedInUser) {
      const foundUser = JSON.parse(loggedInUser);
      this.calculateTotalScore(foundUser.attemptedProblems)
      this.setState({user: {
        userName: foundUser.name,
        userID: foundUser.userID,
        email: foundUser.email,
        userProfilePictureUrl: foundUser.profilePictureUrl,
        isAdmin: foundUser.isAdmin, //check on server
        totalScore: foundUser.totalScore, //check on server
        attemptedProblems: foundUser.attemptedProblems
      }})
    } else {
      this.setState({user: {
        userName: this.loggedOutUserObject.userName,
        userID: this.loggedOutUserObject.userID,
        email: this.loggedOutUserObject.email,
        userProfilePictureUrl: this.loggedOutUserObject.userProfilePictureUrl,
        isAdmin: this.loggedOutUserObject.isAdmin,
        totalScore: this.loggedOutUserObject.totalScore,
        attemptedProblems: this.loggedOutUserObject.attemptedProblems
      }})
    }
    
  }

  verifyAdmin() {
    return this.state.user.isAdmin
  }

  logOut() {
    this.setState({user: {
      userName: this.loggedOutUserObject.userName,
      userID: this.loggedOutUserObject.userID,
      email: this.loggedOutUserObject.email,
      userProfilePictureUrl: this.loggedOutUserObject.userProfilePictureUrl,
      isAdmin: this.loggedOutUserObject.isAdmin,
      totalScore: this.loggedOutUserObject.totalScore,
      attemptedProblems: this.loggedOutUserObject.attemptedProblems
    }})
    localStorage.clear()
  }

  logIn(newUserData) {
    this.calculateTotalScore(newUserData.attemptedProblems)
    this.setState({user: {
      userName: newUserData.name,
      userID: newUserData.userID,
      email: newUserData.email,
      userProfilePictureUrl: newUserData.profilePictureUrl,
      isAdmin: newUserData.isAdmin,
      // totalScore: userTotalScore,
      attemptedProblems: newUserData.attemptedProblems
    }}, () => {
      console.log("this:", this)
    })
    localStorage.setItem("user", JSON.stringify(newUserData))
  }

  calculateTotalScore(attemptedProblems) {
    let solvedProblemIds = []
    for (const property in attemptedProblems) {
      if (attemptedProblems[property].solved) {
        solvedProblemIds.push(Number.parseInt(property))
      }
    }

    Axios.post("http://localhost:3002/getProblems", {filter:{
      questionID : { "$in": solvedProblemIds }
      }}).then((response) => {
        let calculatedScore = 0
        response.data.result.forEach((solvedProblem) => {
          calculatedScore += solvedProblem.beef
        });
        this.setState({user: {...this.state.user, totalScore: calculatedScore}})
    })
  }
  
  render() {
    const styles = {
      app: {
        fontFamily: 'Inter',
        backgroundColor: colors.grey,
      }
    }
  
    const value = {
      user: this.state.user,
      logOutUser: this.logOut,
      logInUser: this.logIn
    }

    return (
      <userContext.Provider value={value}>
      <main style={styles.app}>
        <Routes>
          <Route exact path='/' element={<Landing />} />
          <Route element={<HeaderSkip />}>
            <Route path='/problems' element={<Problems />}  />
            <Route path='/user/:userName' element={<User />} />
            { this.verifyAdmin() == true
            ? <Route path='/create' element={<Create />}  />
            : <Route path='/create' element={<Error />}  />
            }
            <Route path='/problems/:problemName' element={<Question />} />
            <Route path="*" element={<Error />} />
          </Route>
        </Routes>
      </main>
      </userContext.Provider>
    );
    }
}

const Root = () => <Router><App /></Router>;

export default Root;