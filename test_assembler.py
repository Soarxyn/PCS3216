from sisprog import  print_debug, assemble, link

if __name__ == "__main__":
    assemblyResult = assemble("ex.qck", "ex.bdc")[1]
    if assemblyResult == "Assembly successful":
        linkingResult = link(["ex.bdc"], "ex.fita")[1]
        if linkingResult == "Linking successful":
            print_debug("ex.fita")
        else:
            print(linkingResult)
    else:
        print(assemblyResult)